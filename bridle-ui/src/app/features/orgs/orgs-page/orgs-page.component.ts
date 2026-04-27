import {
  Component,
  ChangeDetectionStrategy,
  inject,
  OnInit,
  signal,
} from '@angular/core';

import { OrgService } from '../../../core/services/org.service';
import { NotificationService } from '../../../core/services/notification.service';
import { OrgListComponent } from '../org-list/org-list.component';
import { IngestOrgFormComponent } from '../ingest-org-form/ingest-org-form.component';
import { RepoListComponent } from '../repo-list/repo-list.component';
import { Organization } from '../../../core/models/models';

/**
 * Main page component for managing organizations and repositories.
 */
@Component({
  selector: 'app-orgs-page',
  imports: [
    OrgListComponent,
    IngestOrgFormComponent,
    RepoListComponent
],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="container-lg">
      <div class="mb-4">
        <h1 class="mb-2">Organizations & Repositories</h1>
        <p class="text-muted">
          Ingest and manage source code repositories for local or batch
          processing.
        </p>
      </div>

      <div class="layout-grid">
        <div class="left-col">
          <div class="mb-4">
            <app-ingest-org-form
              (ingest)="onIngest($event)"
            ></app-ingest-org-form>
          </div>

          @if (isLoading()) {
            <p class="text-muted">Loading...</p>
          } @else {
            <app-org-list [orgs]="orgs()" (select)="onSelectOrg($event)">
            </app-org-list>
          }
        </div>

        <div class="right-col">
          @if (selectedOrgId()) {
            @if (isLoadingRepos()) {
              <p class="text-muted">Loading repositories...</p>
            } @else {
              <app-repo-list [orgName]="selectedOrgName()" [repos]="repos()">
              </app-repo-list>
            }
          } @else {
            <div class="blankslate">
              <svg
                height="24"
                aria-hidden="true"
                viewBox="0 0 16 16"
                version="1.1"
                width="24"
                class="octicon octicon-repo mb-3 color-fg-muted"
              >
                <path
                  d="M2 2.5A2.5 2.5 0 0 1 4.5 0h8.75a.75.75 0 0 1 .75.75v12.5a.75.75 0 0 1-.75.75h-2.5a.75.75 0 0 1 0-1.5h1.75v-2h-8a1 1 0 0 0-.714 1.7.75.75 0 1 1-1.072 1.05A2.495 2.495 0 0 1 2 11.5Zm10.5-1h-8a1 1 0 0 0-1 1v6.708A2.486 2.486 0 0 1 4.5 9h8ZM5 12.25a.25.25 0 0 1 .25-.25h3.5a.25.25 0 0 1 .25.25v3.25a.25.25 0 0 1-.4.2l-1.45-1.087a.249.249 0 0 0-.3 0L5.4 15.7a.25.25 0 0 1-.4-.2Z"
                ></path>
              </svg>
              <h2 class="mb-1">Select an organization</h2>
              <p class="text-muted">
                Choose an organization from the list to view its ingested
                repositories.
              </p>
            </div>
          }
        </div>
      </div>
    </div>
  `,
  styles: `
    .mb-1 {
      margin-bottom: 4px;
    }
    .mb-2 {
      margin-bottom: 8px;
    }
    .mb-3 {
      margin-bottom: 16px;
    }
    .mb-4 {
      margin-bottom: 24px;
    }
    .text-muted {
      color: var(--color-fg-muted);
    }
    .color-fg-muted {
      fill: var(--color-fg-muted);
    }

    .layout-grid {
      display: grid;
      grid-template-columns: 1fr 2fr;
      gap: 24px;
      align-items: start;
    }

    @media (max-width: 768px) {
      .layout-grid {
        grid-template-columns: 1fr;
      }
    }

    .blankslate {
      position: relative;
      padding: 32px;
      text-align: center;
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }
  `,
})
export class OrgsPageComponent implements OnInit {
  /** OrgService instance */
  private orgService = inject(OrgService);
  /** NotificationService instance */
  private notificationService = inject(NotificationService);

  /** List of ingested organizations */
  orgs = this.orgService.orgs;
  /** List of repositories for the selected org */
  repos = this.orgService.repos;

  /** Whether orgs are currently loading */
  isLoading = this.orgService.isLoading;

  /** Selected organization ID */
  selectedOrgId = signal<string | null>(null);
  /** Loading state for repos specifically */
  isLoadingRepos = signal<boolean>(false);

  /** Load organizations on init */
  ngOnInit(): void {
    this.orgService.loadOrgs().subscribe({
      error: () =>
        this.notificationService.error('Failed to load organizations'),
    });
  }

  /**
   * Handles the ingestion of a new organization.
   */
  onIngest(data: { name: string; provider: string; dbUrl: string }): void {
    this.orgService.ingestOrg(data.name, data.provider, data.dbUrl).subscribe({
      next: () =>
        this.notificationService.success(
          `Organization ${data.name} ingested successfully`,
        ),
      error: () =>
        this.notificationService.error(
          `Failed to ingest organization ${data.name}`,
        ),
    });
  }

  /**
   * Handles selection of an organization.
   * @param id Organization ID
   */
  onSelectOrg(id: string): void {
    this.selectedOrgId.set(id);
    this.isLoadingRepos.set(true);
    this.orgService.loadRepos(id).subscribe({
      next: () => this.isLoadingRepos.set(false),
      error: () => {
        this.isLoadingRepos.set(false);
        this.notificationService.error('Failed to load repositories');
      },
    });
  }

  /**
   * Computes the name of the currently selected organization.
   * @returns The name of the selected organization, or empty string.
   */
  selectedOrgName(): string {
    const id = this.selectedOrgId();
    if (!id) return '';
    const org = this.orgs().find((o: Organization) => o.id === id);
    return org ? org.name : '';
  }
}
