import { computed, inject, Injectable, signal } from '@angular/core';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { Subject, switchMap, tap, catchError, of } from 'rxjs';
import { PullRequest } from '../models/models';
import { PrService } from '../services/pr.service';
import { NotificationService } from '../services/notification.service';

/** Interface representing the state of pull requests */
export interface PrsState {
  /** List of pull requests */
  prs: PullRequest[];
  /** Syncing state */
  isSyncing: boolean;
  /** Loading state */
  isLoading: boolean;
  /** Error message */
  error: string | null;
  /** Active organization ID */
  activeOrgId: string | null;
}

/** Store for managing pull requests */
@Injectable({ providedIn: 'root' })
export class PrsStore {
  /** PrService instance */
  private prService = inject(PrService);
  /** NotificationService instance */
  private notificationService = inject(NotificationService);

  /** Internal signal holding the state */
  private state = signal<PrsState>({
    prs: [],
    isSyncing: false,
    isLoading: false,
    error: null,
    activeOrgId: null,
  });

  /** Computed list of pull requests */
  readonly prs = computed(() => this.state().prs);
  /** Computed syncing state */
  readonly isSyncing = computed(() => this.state().isSyncing);
  /** Computed loading state */
  readonly isLoading = computed(() => this.state().isLoading);
  /** Computed error state */
  readonly error = computed(() => this.state().error);
  /** Computed active organization ID */
  readonly activeOrgId = computed(() => this.state().activeOrgId);

  /** Computed total number of pull requests */
  readonly totalPrs = computed(() => this.prs().length);
  /** Computed list of synced pull requests */
  readonly syncedPrs = computed(() =>
    this.prs().filter((pr) => pr.status === 'SYNCED'),
  );
  /** Computed list of local pull requests */
  readonly localPrs = computed(() => this.prs().filter((pr) => pr.status === 'LOCAL'));
  /** Computed list of conflicting pull requests */
  readonly conflictPrs = computed(() =>
    this.prs().filter((pr) => pr.status === 'CONFLICT'),
  );

  /** Subject to trigger loading pull requests */
  private loadPrsSubject = new Subject<string>();
  /** Subject to trigger syncing pull requests */
  private syncPrsSubject = new Subject<{ orgId: string; maxRate: number }>();

  /** Constructor initializes the streams for loading and syncing PRs */
  constructor() {
    this.loadPrsSubject
      .pipe(
        takeUntilDestroyed(),
        tap((orgId) =>
          this.patchState({
            isLoading: true,
            error: null,
            activeOrgId: orgId,
          }),
        ),
        switchMap((orgId) => {
          return this.prService.loadPrs(orgId).pipe(
            tap((prs) => this.patchState({ prs, isLoading: false })),
            catchError((err) => {
              const errorMsg = err.message || 'Failed to load PRs';
              this.patchState({ error: errorMsg, isLoading: false });
              this.notificationService.error(errorMsg);
              return of([]);
            }),
          );
        }),
      )
      .subscribe();

    this.syncPrsSubject
      .pipe(
        takeUntilDestroyed(),
        tap(() => this.patchState({ isSyncing: true })),
        switchMap(({ orgId, maxRate }) => {
          return this.prService.syncPrs(orgId, maxRate).pipe(
            tap(({ syncedCount }) => {
              this.patchState({ isSyncing: false });
              this.notificationService.success(
                `Successfully synced ${syncedCount} PRs.`,
              );
              // Reload PRs after successful sync
              if (this.activeOrgId() === orgId) {
                this.loadPrs(orgId);
              }
            }),
            catchError((err) => {
              this.patchState({ isSyncing: false });
              this.notificationService.error(err.message || 'Failed to sync PRs');
              return of(null);
            }),
          );
        }),
      )
      .subscribe();
  }

  /**
   * Triggers loading PRs for a specific organization
   * @param orgId The organization ID
   */
  loadPrs(orgId: string) {
    this.loadPrsSubject.next(orgId);
  }

  /**
   * Triggers syncing PRs for an organization
   * @param params Object containing orgId and maxRate
   * @param params.orgId The organization ID
   * @param params.maxRate The maximum rate to sync
   */
  syncPrs(params: { orgId: string; maxRate: number }) {
    this.syncPrsSubject.next(params);
  }

  /**
   * Partially updates the internal state
   * @param partialState The state changes to apply
   */
  private patchState(partialState: Partial<PrsState>) {
    this.state.update((state) => ({ ...state, ...partialState }));
  }
}
