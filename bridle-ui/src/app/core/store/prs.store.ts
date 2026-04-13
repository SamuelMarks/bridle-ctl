import { computed, inject } from '@angular/core';
import {
  signalStore,
  withState,
  withMethods,
  withComputed,
  patchState,
} from '@ngrx/signals';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { pipe, switchMap, tap, catchError, of } from 'rxjs';
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

/** Initial state for PRs store */
const initialState: PrsState = {
  prs: [],
  isSyncing: false,
  isLoading: false,
  error: null,
  activeOrgId: null,
};

/** Signal store for managing pull requests */
export const PrsStore = signalStore(
  { providedIn: 'root' },
  withState(initialState),
  withComputed((store) => ({
    totalPrs: computed(() => store.prs().length),
    syncedPrs: computed(() =>
      store.prs().filter((pr) => pr.status === 'SYNCED'),
    ),
    localPrs: computed(() => store.prs().filter((pr) => pr.status === 'LOCAL')),
    conflictPrs: computed(() =>
      store.prs().filter((pr) => pr.status === 'CONFLICT'),
    ),
  })),
  withMethods(
    (
      store,
      prService = inject(PrService),
      notificationService = inject(NotificationService),
    ) => {
      // Define loadPrs internally first so we can use it within other methods
      const loadPrsFn = rxMethod<string>(
        pipe(
          tap((orgId) =>
            patchState(store, {
              isLoading: true,
              error: null,
              activeOrgId: orgId,
            }),
          ),
          switchMap((orgId) => {
            return prService.loadPrs(orgId).pipe(
              tap((prs) => patchState(store, { prs, isLoading: false })),
              catchError((err) => {
                const errorMsg = err.message || 'Failed to load PRs';
                patchState(store, { error: errorMsg, isLoading: false });
                notificationService.error(errorMsg);
                return of([]);
              }),
            );
          }),
        ),
      );

      return {
        loadPrs: loadPrsFn,
        syncPrs: rxMethod<{ orgId: string; maxRate: number }>(
          pipe(
            tap(() => patchState(store, { isSyncing: true })),
            switchMap(({ orgId, maxRate }) => {
              return prService.syncPrs(orgId, maxRate).pipe(
                tap(({ syncedCount }) => {
                  patchState(store, { isSyncing: false });
                  notificationService.success(
                    `Successfully synced ${syncedCount} PRs.`,
                  );
                  // Reload PRs after successful sync
                  if (store.activeOrgId() === orgId) {
                    loadPrsFn(orgId);
                  }
                }),
                catchError((err) => {
                  patchState(store, { isSyncing: false });
                  notificationService.error(
                    err.message || 'Failed to sync PRs',
                  );
                  return of(null);
                }),
              );
            }),
          ),
        ),
      };
    },
  ),
);
