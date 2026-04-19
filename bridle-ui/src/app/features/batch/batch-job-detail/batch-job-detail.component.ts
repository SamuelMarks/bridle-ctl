import {
  Component,
  ChangeDetectionStrategy,
  input,
  output,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';

/**
 * Component to display details of a specific batch job.
 */
@Component({
  selector: 'app-batch-job-detail',
  imports: [CommonModule, AppButtonComponent, AppBadgeComponent],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="Box">
      <div class="Box-header d-flex justify-content-between align-items-center">
        <h2 class="Box-title">Job Details: {{ job()?.id?.substring(0, 8) }}</h2>
        <app-button variant="invisible" (click)="close.emit()">
          <svg
            aria-hidden="true"
            height="16"
            viewBox="0 0 16 16"
            version="1.1"
            width="16"
            class="octicon octicon-x"
          >
            <path
              d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
            ></path>
          </svg>
        </app-button>
      </div>

      <div class="Box-body p-3">
        @if (job(); as j) {
          <div class="d-flex mb-3">
            <div class="flex-1 pr-3">
              <div class="text-bold text-small text-muted mb-1">Target</div>
              <div>{{ j.target }}</div>
            </div>
            <div class="flex-1 px-3">
              <div class="text-bold text-small text-muted mb-1">Status</div>
              <app-badge [variant]="getStatusVariant(j.status)">
                {{ j.status }}
              </app-badge>
            </div>
            <div class="flex-1 pl-3">
              <div class="text-bold text-small text-muted mb-1">Created</div>
              <div>{{ j.createdAt | date: 'medium' }}</div>
            </div>
          </div>

          <div class="mt-4 pt-3 border-top">
            <div class="d-flex justify-content-between align-items-center mb-2">
              <h3 class="h5 m-0">Execution Log</h3>

              @if (j.status === 'INTERRUPTED') {
                <app-button variant="primary" (click)="resume.emit(j.id)">
                  Resume Job
                </app-button>
              }
            </div>

            <pre class="cli-output p-3">
Log tail for job {{ j.id }}...
Status: {{ j.status }}</pre
            >
          </div>
        } @else {
          <p class="text-muted text-center py-4">No job details available.</p>
        }
      </div>
    </div>
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }
    .Box-header {
      padding: 8px 16px;
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
      border-bottom: 1px solid var(--color-border-default);
    }
    .p-3 {
      padding: 16px;
    }
    .px-3 {
      padding-left: 16px;
      padding-right: 16px;
    }
    .pr-3 {
      padding-right: 16px;
    }
    .pl-3 {
      padding-left: 16px;
    }
    .py-4 {
      padding-top: 24px;
      padding-bottom: 24px;
    }
    .pt-3 {
      padding-top: 16px;
    }
    .m-0 {
      margin: 0;
    }
    .mb-1 {
      margin-bottom: 4px;
    }
    .mb-2 {
      margin-bottom: 8px;
    }
    .mb-3 {
      margin-bottom: 16px;
    }
    .mt-4 {
      margin-top: 24px;
    }

    .flex-1 {
      flex: 1;
    }
    .border-top {
      border-top: 1px solid var(--color-border-default);
    }
    .text-center {
      text-align: center;
    }
    .text-muted {
      color: var(--color-fg-muted);
    }
    .text-small {
      font-size: 12px;
    }
    .text-bold {
      font-weight: 600;
    }
    .h5 {
      font-size: 14px;
      font-weight: 600;
    }
    .octicon {
      vertical-align: text-bottom;
      fill: currentColor;
    }

    .cli-output {
      font-family: var(--font-family-mono);
      font-size: 12px;
      line-height: 1.45;
      background-color: var(--color-canvas-subtle);
      color: var(--color-fg-default);
      overflow-x: auto;
      white-space: pre-wrap;
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
      margin: 0;
    }
  `,
})
export class BatchJobDetailComponent {
  /** The batch job to display */
  job = input<BatchJob | null>(null);

  /** Emitted when the detail view is closed */
  close = output<void>();

  /** Emitted to resume an interrupted job */
  resume = output<string>();

  /**
   * Helper to map job status to badge variant.
   * @param status The job status
   * @returns The badge variant
   */
  getStatusVariant(
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
