import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from './api.service';
import { PullRequest } from '../models/models';
import { tap } from 'rxjs/operators';
import { Observable } from 'rxjs';

/**
 * Service for managing pull requests and sync operations.
 */
@Injectable({
  providedIn: 'root',
})
export class PrService {
  /** API Service instance */
  private api = inject(ApiService);

  /** Signal for PRs */
  private prsSignal = signal<PullRequest[]>([]);
  /** Signal for syncing state */
  private isSyncingSignal = signal<boolean>(false);

  /** List of PRs */
  readonly prs = this.prsSignal.asReadonly();
  /** Whether a sync operation is in progress */
  readonly isSyncing = this.isSyncingSignal.asReadonly();

  /**
   * Loads PRs for an organization.
   * @param orgId Organization ID
   */
  loadPrs(orgId: string): Observable<PullRequest[]> {
    return this.api
      .get<PullRequest[]>(`/prs?orgId=${orgId}`)
      .pipe(tap((prs) => this.prsSignal.set(prs)));
  }

  /**
   * Syncs local PRs to upstream.
   * @param orgId Target organization
   * @param maxRate Maximum PRs to sync
   */
  syncPrs(orgId: string, maxRate: number): Observable<{ syncedCount: number }> {
    this.isSyncingSignal.set(true);
    return this.api
      .post<{ syncedCount: number }>('/prs/sync', { orgId, maxRate })
      .pipe(
        tap({
          next: () => this.isSyncingSignal.set(false),
          error: () => this.isSyncingSignal.set(false),
        }),
      );
  }
}
