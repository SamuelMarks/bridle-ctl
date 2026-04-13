import {
  Component,
  ChangeDetectionStrategy,
  inject,
  OnInit,
  computed,
  DestroyRef,
} from '@angular/core';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { ScrollingModule } from '@angular/cdk/scrolling';
import { OrgService } from '../../../core/services/org.service';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';
import { PrsStore } from '../../../core/store/prs.store';
import { PullRequest } from '../../../core/models/models';

/**
 * Component for synchronizing pull requests.
 * Uses NgRx SignalStore to handle large volumes of PRs with CDK Virtual Scroll.
 */
@Component({
  selector: 'app-pr-sync',
  imports: [
    CommonModule,
    ReactiveFormsModule,
    ScrollingModule,
    AppButtonComponent,
    AppInputComponent,
    AppBadgeComponent,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="container-lg">
      <div class="mb-4">
        <h2 class="mb-2">Pull Requests Synchronization</h2>
        <p class="text-muted">
          Review local pull requests and sync them to the upstream source
          control provider.
        </p>
      </div>

      <div class="Box p-3 mb-4">
        <form
          [formGroup]="form"
          (ngSubmit)="onSync()"
          class="d-flex align-items-end gap-3 form-row"
        >
          <div class="flex-1">
            <app-input
              formControlName="orgId"
              label="Organization"
              type="select"
              [options]="orgOptions()"
            ></app-input>
          </div>

          <div class="flex-1">
            <app-input
              formControlName="maxRate"
              label="Max PRs per hour"
              type="number"
              placeholder="e.g. 5"
            ></app-input>
          </div>

          <div>
            <app-button
              type="submit"
              variant="primary"
              [disabled]="
                form.invalid ||
                prsStore.isSyncing() ||
                prsStore.prs().length === 0
              "
            >
              {{ prsStore.isSyncing() ? 'Syncing...' : 'Sync to Upstream' }}
            </app-button>
          </div>
        </form>
      </div>

      <div class="summary-stats mb-3">
        <span class="mr-3"
          ><strong>Total PRs:</strong> {{ prsStore.totalPrs() }}</span
        >
        <span class="mr-3"
          ><strong>Local:</strong> {{ prsStore.localPrs().length }}</span
        >
        <span class="mr-3"
          ><strong>Synced:</strong> {{ prsStore.syncedPrs().length }}</span
        >
        <span class="mr-3 text-danger"
          ><strong>Conflicts:</strong> {{ prsStore.conflictPrs().length }}</span
        >
      </div>

      <div class="list-container">
        <div class="list-header">
          <h3 class="m-0">Local PRs Pending Sync</h3>
        </div>

        @if (prsStore.isLoading()) {
          <div class="p-4 text-center text-muted">Loading Pull Requests...</div>
        } @else if (prsStore.prs().length === 0) {
          <div class="p-4 text-center text-muted">
            No PRs found for the selected organization.
          </div>
        } @else {
          <cdk-virtual-scroll-viewport itemSize="64" class="viewport">
            <div
              *cdkVirtualFor="let pr of prsStore.prs(); trackBy: trackById"
              class="list-item"
            >
              <div class="item-info">
                <span class="text-bold color-fg-default">
                  #{{ pr.id.substring(0, 8) }} - {{ pr.title }}
                </span>
                <span class="text-muted text-small ml-2"
                  >Repo: {{ pr.repoId }}</span
                >
              </div>

              <div class="item-meta">
                <app-badge
                  [variant]="
                    pr.status === 'LOCAL'
                      ? 'default'
                      : pr.status === 'SYNCED'
                        ? 'success'
                        : 'danger'
                  "
                >
                  {{ pr.status }}
                </app-badge>
              </div>
            </div>
          </cdk-virtual-scroll-viewport>
        }
      </div>
    </div>
  `,
  styles: `
    .mb-2 {
      margin-bottom: 8px;
    }
    .mb-3 {
      margin-bottom: 16px;
    }
    .mb-4 {
      margin-bottom: 24px;
    }
    .m-0 {
      margin: 0;
    }
    .p-3 {
      padding: 16px;
    }
    .p-4 {
      padding: 24px;
    }
    .mr-3 {
      margin-right: 16px;
    }
    .ml-2 {
      margin-left: 8px;
    }
    .text-center {
      text-align: center;
    }
    .text-muted {
      color: var(--color-fg-muted, #57606a);
    }
    .text-bold {
      font-weight: 600;
    }
    .text-small {
      font-size: 12px;
    }
    .text-danger {
      color: var(--color-danger-fg, #cf222e);
    }
    .color-fg-default {
      color: var(--color-fg-default, #24292f);
    }

    .Box {
      background-color: var(--color-canvas-default, #ffffff);
      border: 1px solid var(--color-border-default, #d0d7de);
      border-radius: 6px;
    }

    .flex-1 {
      flex: 1;
    }
    .gap-3 {
      gap: 16px;
    }
    .d-flex {
      display: flex;
    }
    .align-items-end {
      align-items: flex-end;
    }

    .summary-stats {
      font-size: 14px;
      color: var(--color-fg-default, #24292f);
    }

    .list-container {
      border: 1px solid var(--color-border-default, #d0d7de);
      border-radius: 6px;
      overflow: hidden;
      background-color: var(--color-canvas-default, #ffffff);
    }
    .list-header {
      padding: 16px;
      border-bottom: 1px solid var(--color-border-default, #d0d7de);
      background-color: var(--color-canvas-subtle, #f6f8fa);
    }

    .viewport {
      height: 600px;
      width: 100%;
    }

    .list-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 0 16px;
      height: 64px;
      border-bottom: 1px solid var(--color-border-subtle, #ebf0f4);
      box-sizing: border-box;
    }
    .list-item:hover {
      background-color: var(--color-canvas-subtle, #f6f8fa);
    }

    @media (max-width: 768px) {
      .form-row {
        flex-direction: column;
        align-items: stretch;
      }
    }
  `,
})
export class PrSyncComponent implements OnInit {
  /** FormBuilder instance */
  private fb = inject(FormBuilder);
  /** OrgService instance */
  private orgService = inject(OrgService);
  /** PrsStore instance */
  prsStore = inject(PrsStore);

  /** Form model */
  form = this.fb.group({
    orgId: ['', Validators.required],
    maxRate: [10, [Validators.required, Validators.min(1)]],
  });

  /** Computed options for the org select */
  orgOptions = computed(() => {
    const orgs = this.orgService.orgs();
    return [
      { label: '-- Select Organization --', value: '' },
      ...orgs.map((o) => ({ label: o.name, value: o.id })),
    ];
  });

  /** DestroyRef instance */
  private destroyRef = inject(DestroyRef);

  /** Initialize component */
  ngOnInit(): void {
    // Load orgs if not loaded
    if (this.orgService.orgs().length === 0) {
      this.orgService
        .loadOrgs()
        .pipe(takeUntilDestroyed(this.destroyRef))
        .subscribe();
    }

    // Listen to orgId changes and load PRs via the store rxMethod
    this.form
      .get('orgId')
      ?.valueChanges.pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((orgId) => {
        if (orgId) {
          this.prsStore.loadPrs(orgId);
        }
      });
  }

  /**
   * Track by function for virtual scroll list
   */
  trackById(index: number, pr: PullRequest): string {
    return pr.id;
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
