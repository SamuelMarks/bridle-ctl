import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from './api.service';
import { Organization, Repository } from '../models/models';
import { tap } from 'rxjs/operators';
import { Observable } from 'rxjs';

/**
 * Service for managing organizations and repositories.
 */
@Injectable({
  providedIn: 'root'
})
export class OrgService {
  /** API Service instance */
  private api = inject(ApiService);
  
  /** Signal for orgs */
  private orgsSignal = signal<Organization[]>([]);
  /** Signal for repos */
  private reposSignal = signal<Repository[]>([]);
  /** Signal for loading state */
  private isLoadingSignal = signal<boolean>(false);

  /** List of organizations */
  readonly orgs = this.orgsSignal.asReadonly();
  /** List of repositories for the selected org */
  readonly repos = this.reposSignal.asReadonly();
  /** Loading state */
  readonly isLoading = this.isLoadingSignal.asReadonly();

  /**
   * Fetches all ingested organizations.
   */
  loadOrgs(): Observable<Organization[]> {
    this.isLoadingSignal.set(true);
    return this.api.get<Organization[]>('/orgs').pipe(
      tap({
        next: (orgs) => {
          this.orgsSignal.set(orgs);
          this.isLoadingSignal.set(false);
        },
        error: () => this.isLoadingSignal.set(false)
      })
    );
  }

  /**
   * Ingests a new organization.
   * @param name Org name
   * @param provider Provider
   * @param dbUrl Database URL
   */
  ingestOrg(name: string, provider: string, dbUrl: string): Observable<Organization> {
    this.isLoadingSignal.set(true);
    return this.api.post<Organization>('/orgs/ingest', { name, provider, dbUrl }).pipe(
      tap({
        next: (org) => {
          this.orgsSignal.update(orgs => [...orgs, org]);
          this.isLoadingSignal.set(false);
        },
        error: () => this.isLoadingSignal.set(false)
      })
    );
  }

  /**
   * Loads repositories for an organization.
   * @param orgId Organization ID
   */
  loadRepos(orgId: string): Observable<Repository[]> {
    this.isLoadingSignal.set(true);
    return this.api.get<Repository[]>(`/orgs/${orgId}/repos`).pipe(
      tap({
        next: (repos) => {
          this.reposSignal.set(repos);
          this.isLoadingSignal.set(false);
        },
        error: () => this.isLoadingSignal.set(false)
      })
    );
  }
}
