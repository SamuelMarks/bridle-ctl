import {
  Component,
  inject,
  OnInit,
  computed,
  DestroyRef,
} from '@angular/core';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { PrsStore } from '../../../core/store/prs.store';
import { OrgService } from '../../../core/services/org.service';
import { NotificationService } from '../../../core/services/notification.service';
import { Organization } from '../../../core/models/models';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';
import {
  AppTableComponent,
  AppTableColumnDirective,
} from '../../../shared/ui/app-table/app-table.component';

/**
 * Component for synchronizing local pull requests with upstream providers.
 */
@Component({
  selector: 'app-pr-sync',
  imports: [
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent,
    AppBadgeComponent,
    AppTableComponent,
    AppTableColumnDirective,
  ],
  template: `
    <div class="pr-sync-container">
      <div class="page-header">
        <h1 class="h2">PR Synchronization</h1>
      </div>

      <div class="Box mb-4">
        <div class="Box-header">
          <h2 class="Box-title">Sync Controls</h2>
        </div>
        <div class="Box-body">
          <form [formGroup]="form" (ngSubmit)="onSync()" class="sync-form">
            <div class="form-row">
              <app-input
                type="select"
                label="Select Organization"
                [options]="orgOptions()"
                formControlName="orgId"
                class="flex-1"
              ></app-input>

              <app-input
                type="number"
                label="Max Rate (PRs / hour)"
                placeholder="10"
                formControlName="maxRate"
                class="rate-input"
              ></app-input>

              <app-input
                type="text"
                label="Fork to Org (Optional)"
                placeholder="e.g. my-fork-org"
                formControlName="forkOrg"
                class="flex-1"
              ></app-input>
            </div>

            <div class="form-actions">
              <app-button
                type="submit"
                variant="primary"
                [disabled]="form.invalid || isSyncing()"
              >
                {{ isSyncing() ? 'Syncing PRs...' : 'Sync PRs Now' }}
              </app-button>
            </div>
          </form>
        </div>
      </div>

      <app-table
        title="Pull Requests"
        [data]="prs()"
        emptyMessage="No pull requests found for the selected organization."
      >
        <ng-template appTableColumn title="ID" key="id" let-value="value">
          <span class="font-mono">#{{ value }}</span>
        </ng-template>
        <ng-template appTableColumn title="Title" key="title" let-pr>
          <strong>{{ pr.title }}</strong>
        </ng-template>
        <ng-template
          appTableColumn
          title="Repository"
          key="repoId"
          let-value="value"
        >
          <span class="text-muted">{{ value }}</span>
        </ng-template>
        <ng-template appTableColumn title="Status" key="status" let-pr>
          <app-badge
            [variant]="
              pr.status === 'SYNCED'
                ? 'success'
                : pr.status === 'CONFLICT'
                  ? 'danger'
                  : 'accent'
            "
          >
            {{ pr.status }}
          </app-badge>
        </ng-template>
      </app-table>
    </div>
  `,
  styles: `
    .pr-sync-container {
      max-width: 1024px;
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

    .mb-4 {
      margin-bottom: 24px;
    }

    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }

    .Box-header {
      padding: 16px;
      background-color: var(--color-canvas-subtle);
      border-bottom: 1px solid var(--color-border-default);
      border-top-left-radius: var(--border-radius-2);
      border-top-right-radius: var(--border-radius-2);
    }

    .Box-title {
      font-size: 14px;
      font-weight: 600;
      margin: 0;
    }

    .Box-body {
      padding: 16px;
    }

    .sync-form {
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    .form-row {
      display: flex;
      gap: 16px;
      align-items: flex-start;
    }

    .flex-1 {
      flex: 1;
    }

    .rate-input {
      width: 200px;
    }

    .form-actions {
      display: flex;
      justify-content: flex-end;
    }

    .font-mono {
      font-family: var(--font-mono);
      font-size: 12px;
    }

    .text-muted {
      color: var(--color-fg-muted);
    }
  `,
})
export class PrSyncComponent implements OnInit {
  /** Injected FormBuilder */
  private fb = inject(FormBuilder);
  /** Injected PrsStore */
  private prsStore = inject(PrsStore);
  /** Injected OrgService */
  private orgService = inject(OrgService);
  /** Injected NotificationService */
  private notificationService = inject(NotificationService);
  /** Injected DestroyRef */
  private destroyRef = inject(DestroyRef);

  /** Signal of PRs */
  prs = this.prsStore.prs;
  /** Signal of syncing status */
  isSyncing = this.prsStore.isSyncing;

  /** Computed options for organization select */
  orgOptions = computed(() => {
    return this.orgService.orgs().map((o: Organization) => ({
      label: o.name,
      value: o.id,
    }));
  });

  /** Reactive form group */
  form = this.fb.group({
    orgId: ['', Validators.required],
    maxRate: [10, [Validators.required, Validators.min(1)]],
    forkOrg: [''],
  });

  /** Initializes component and sets up org listeners */
  ngOnInit(): void {
    this.orgService.loadOrgs().subscribe({
      next: (orgs: Organization[]) => {
        if (orgs.length > 0 && !this.form.value.orgId) {
          this.form.patchValue({ orgId: orgs[0].id });
          this.loadPrs(orgs[0].id);
        }
      },
      error: () =>
        this.notificationService.error('Failed to load organizations'),
    });

    this.form.controls.orgId.valueChanges
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((orgId) => {
        if (orgId) {
          this.loadPrs(orgId);
        }
      });
  }

  /**
   * Loads PRs for the given organization.
   * @param orgId The organization ID
   */
  loadPrs(orgId: string): void {
    this.prsStore.loadPrs(orgId);
  }

  /** Handles the sync action */
  onSync(): void {
    if (this.form.valid) {
      const orgId = this.form.value.orgId!;
      const maxRate = this.form.value.maxRate!;
      this.prsStore.syncPrs({ orgId, maxRate });
    }
  }
}
