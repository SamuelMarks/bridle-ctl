import { Component, input, output } from '@angular/core';
import { DatePipe } from '@angular/common';
import { ScrollingModule } from '@angular/cdk/scrolling';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';

/**
 * Component displaying a table of batch jobs using virtual scrolling.
 */
@Component({
  selector: 'app-batch-jobs-list',
  imports: [DatePipe, ScrollingModule, AppBadgeComponent],
  template: `
    <div class="Box">
      <div class="Box-header">
        <h2 class="Box-title">Batch Jobs</h2>
      </div>
      <div class="Box-body p-0">
        @if (jobs().length === 0) {
          <div class="empty-state text-center text-muted p-3">
            No batch jobs found.
          </div>
        } @else {
          <div class="table-header">
            <div class="col-id">Job ID</div>
            <div class="col-target">Target</div>
            <div class="col-status">Status</div>
            <div class="col-created">Created</div>
            <div class="col-actions">Actions</div>
          </div>
          <cdk-virtual-scroll-viewport itemSize="45" class="virtual-viewport">
            <div *cdkVirtualFor="let job of jobs()" class="table-row">
              <div class="col-id font-mono">{{ job.id }}</div>
              <div class="col-target">{{ job.target }}</div>
              <div class="col-status">
                <app-badge [variant]="getBadgeVariant(job.status)">
                  {{ job.status }}
                </app-badge>
              </div>
              <div class="col-created text-muted">
                {{ job.createdAt | date: 'short' }}
              </div>
              <div class="col-actions">
                <button
                  type="button"
                  class="btn-link"
                  (click)="selectJob.emit(job.id)"
                >
                  Details
                </button>
              </div>
            </div>
          </cdk-virtual-scroll-viewport>
        }
      </div>
    </div>
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
      overflow: hidden;
    }

    .Box-header {
      padding: 16px;
      margin: -1px -1px 0;
      background-color: var(--color-canvas-subtle);
      border: 1px solid var(--color-border-default);
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

    .p-0 {
      padding: 0 !important;
    }

    .p-3 {
      padding: 16px !important;
    }

    .text-center {
      text-align: center !important;
    }

    .text-muted {
      color: var(--color-fg-muted) !important;
    }

    .font-mono {
      font-family: var(--font-mono);
      font-size: 12px;
    }

    .btn-link {
      background: none;
      border: 0;
      padding: 0;
      cursor: pointer;
      font-size: 12px;
      font-weight: 500;
      color: var(--color-accent-fg);
    }

    .btn-link:hover {
      text-decoration: underline;
    }

    .table-header {
      display: flex;
      background-color: var(--color-canvas-subtle);
      color: var(--color-fg-muted);
      font-weight: 600;
      font-size: 12px;
      border-bottom: 1px solid var(--color-border-muted);
      padding: 8px 16px;
    }

    .virtual-viewport {
      height: 400px;
      width: 100%;
    }

    .table-row {
      display: flex;
      align-items: center;
      padding: 0 16px;
      height: 45px;
      border-bottom: 1px solid var(--color-border-muted);
      box-sizing: border-box;
    }

    .table-row:hover {
      background-color: var(--color-canvas-subtle);
    }

    .col-id {
      flex: 2;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .col-target {
      flex: 2;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .col-status {
      flex: 1.5;
    }
    .col-created {
      flex: 2;
    }
    .col-actions {
      flex: 1;
      text-align: right;
    }
  `,
})
export class BatchJobsListComponent {
  /** List of batch jobs */
  jobs = input<BatchJob[]>([]);

  /** Emitted when a job is selected */
  selectJob = output<string>();

  /**
   * Maps job status to badge variant.
   * @param status The job status
   * @returns The badge variant
   */
  getBadgeVariant(
    status: BatchJobStatus,
  ): 'default' | 'success' | 'danger' | 'accent' {
    switch (status) {
      case BatchJobStatus.COMPLETED:
        return 'success';
      case BatchJobStatus.FAILED:
        return 'danger';
      case BatchJobStatus.RUNNING:
        return 'accent';
      default:
        return 'default';
    }
  }
}
