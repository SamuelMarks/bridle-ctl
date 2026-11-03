import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { PullRequest } from '../models/models';
import { Observable } from 'rxjs';

/**
 * Stateless HTTP data-access service for Pull Request operations.
 * Reactive state is managed by `PrsStore`.
 */
@Injectable({
  providedIn: 'root',
})
export class PrService {
  /** Injected API service */
  private api = inject(ApiService);

  /**
   * Loads pull requests for a specific organization.
   * @param orgId The organization ID
   * @returns Observable of PullRequest array
   */
  loadPrs(orgId: string): Observable<PullRequest[]> {
    return this.api.get<PullRequest[]>(`/prs?orgId=${orgId}`);
  }

  /**
   * Syncs local pull requests with the upstream provider.
   * @param orgId The organization ID
   * @param maxRate Rate limit (PRs per minute)
   * @param forkOrg Specific organization to fork to
   * @returns Observable of sync result
   */
  syncPrs(
    orgId: string,
    maxRate: number,
    forkOrg?: string,
  ): Observable<{ syncedCount: number }> {
    return this.api.post<{ syncedCount: number }>('/prs/sync', {
      org: orgId,
      max_prs_per_hour: maxRate,
      fork_org: forkOrg,
    });
  }
}
