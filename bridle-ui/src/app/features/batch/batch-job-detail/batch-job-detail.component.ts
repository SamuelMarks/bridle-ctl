import { Component, input, output } from '@angular/core';
import { DatePipe } from '@angular/common';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';

/**
 * Component displaying detailed status of a batch job.
 */
@Component({
  selector: 'app-batch-job-detail',
  imports: [DatePipe, AppButtonComponent, AppBadgeComponent],
  template: `
    @if (job()) {
      <div class="Box">
        <div class="Box-header d-flex flex-justify-between flex-items-center">
          <h2 class="Box-title">Job Details: {{ job()!.id }}</h2>
          <app-button variant="invisible" (click)="close.emit()">
            &times; Close
          </app-button>
        </div>
        <div class="Box-body">
          <div class="detail-row">
            <span class="detail-label">Target:</span>
            <span>{{ job()!.target }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Status:</span>
            <app-badge [variant]="getBadgeVariant(job()!.status)">
              {{ job()!.status }}
            </app-badge>
          </div>
          <div class="detail-row">
            <span class="detail-label">Created At:</span>
            <span>{{ job()!.createdAt | date: 'medium' }}</span>
          </div>

          @if (
            job()!.status === BatchJobStatus.INTERRUPTED ||
            job()!.status === BatchJobStatus.FAILED
          ) {
            <div class="mt-4 pt-3 border-top">
              <app-button variant="primary" (click)="resume.emit(job()!.id)">
                Resume Interrupted Job
              </app-button>
            </div>
          }
        </div>
      </div>
    }
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }

    .Box-header {
      padding: 12px 16px;
      background-color: var(--color-canvas-subtle);
      border-bottom: 1px solid var(--color-border-default);
      border-top-left-radius: var(--border-radius-2);
      border-top-right-radius: var(--border-radius-2);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .Box-title {
      font-size: 14px;
      font-weight: 600;
      margin: 0;
    }

    .Box-body {
      padding: 16px;
    }

    .detail-row {
      display: flex;
      align-items: center;
      padding: 8px 0;
      border-bottom: 1px solid var(--color-border-muted);
      font-size: 14px;
    }

    .detail-row:last-of-type {
      border-bottom: 0;
    }

    .detail-label {
      width: 120px;
      font-weight: 600;
      color: var(--color-fg-muted);
    }

    .mt-4 {
      margin-top: 16px;
    }
    .pt-3 {
      padding-top: 12px;
    }
    .border-top {
      border-top: 1px solid var(--color-border-default);
    }
  `,
})
export class BatchJobDetailComponent {
  /** Expose enum to template */
  BatchJobStatus = BatchJobStatus;

  /** The job to display */
  job = input<BatchJob | null>(null);

  /** Emitted when resume is clicked */
  resume = output<string>();

  /** Emitted when close is clicked */
  close = output<void>();

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
