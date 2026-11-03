import { Component, computed, inject, OnInit, signal } from '@angular/core';
import { OrgService } from '../../../core/services/org.service';
import { NotificationService } from '../../../core/services/notification.service';
import { Organization } from '../../../core/models/models';
import { OrgListComponent } from '../org-list/org-list.component';
import { RepoListComponent } from '../repo-list/repo-list.component';
import { IngestOrgFormComponent } from '../ingest-org-form/ingest-org-form.component';

/**
 * Page component for managing organizations and viewing their repositories.
 */
@Component({
  selector: 'app-orgs-page',
  imports: [OrgListComponent, RepoListComponent, IngestOrgFormComponent],
  template: `
    <div class="orgs-page-container">
      <div class="page-header">
        <h1 class="h2">Organizations & Repositories</h1>
      </div>

      <div class="layout-grid">
        <div class="main-column">
          <app-org-list
            [orgs]="orgs()"
            (select)="onSelectOrg($event)"
          ></app-org-list>

          @if (selectedOrgId()) {
            <div class="mt-4">
              <app-repo-list
                [orgName]="selectedOrgName()"
                [repos]="repos()"
              ></app-repo-list>
            </div>
          }
        </div>

        <div class="sidebar-column">
          <app-ingest-org-form
            (ingest)="onIngestOrg($event)"
          ></app-ingest-org-form>
        </div>
      </div>
    </div>
  `,
  styles: `
    .orgs-page-container {
      max-width: 1200px;
      margin: 0 auto;
    }

    .page-header {
      margin-bottom: 24px;
      padding-bottom: 16px;
      border-bottom: 1px solid var(--color-border-default);
    }

    .h2 {
      font-size: 24px;
      font-weight: 600;
      margin: 0;
    }

    .layout-grid {
      display: grid;
      grid-template-columns: 1fr 340px;
      gap: 24px;
      align-items: start;
    }

    @media (max-width: 768px) {
      .layout-grid {
        grid-template-columns: 1fr;
      }
    }

    .mt-4 {
      margin-top: 24px;
    }
  `,
})
export class OrgsPageComponent implements OnInit {
  /** Injected OrgService */
  private orgService = inject(OrgService);
  /** Injected NotificationService */
  private notificationService = inject(NotificationService);

  /** Signal of organizations */
  orgs = this.orgService.orgs;
  /** Signal of repositories */
  repos = this.orgService.repos;

  /** Currently selected organization ID */
  selectedOrgId = signal<string | null>(null);

  /** Computed name of the currently selected organization */
  readonly selectedOrgName = computed(() => {
    const id = this.selectedOrgId();
    if (!id) return '';
    const org = this.orgs().find((o: Organization) => o.id === id);
    return org ? org.name : '';
  });

  /** Initializes the component and loads organizations */
  ngOnInit(): void {
    this.orgService.loadOrgs().subscribe({
      error: () =>
        this.notificationService.error('Failed to load organizations'),
    });
  }

  /**
   * Handles selecting an organization.
   * @param orgId The selected organization ID
   */
  onSelectOrg(orgId: string): void {
    this.selectedOrgId.set(orgId);
    this.orgService.loadRepos(orgId).subscribe({
      error: () =>
        this.notificationService.error('Failed to load repositories'),
    });
  }

  /**
   * Handles ingesting a new organization.
   * @param data The ingestion form data
   * @param data.name The organization name
   * @param data.provider The upstream provider
   * @param data.dbUrl The database URL
   */
  onIngestOrg(data: { name: string; provider: string; dbUrl: string }): void {
    this.orgService.ingestOrg(data.name, data.provider, data.dbUrl).subscribe({
      next: () => {
        this.notificationService.success(
          `Organization ${data.name} ingested successfully`,
        );
      },
      error: () => {
        this.notificationService.error(
          `Failed to ingest organization ${data.name}`,
        );
      },
    });
  }
}
