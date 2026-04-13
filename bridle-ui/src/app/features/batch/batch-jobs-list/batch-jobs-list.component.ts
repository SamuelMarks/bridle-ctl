import {
  Component,
  ChangeDetectionStrategy,
  input,
  output,
} from '@angular/core';
import { CommonModule, DatePipe } from '@angular/common';
import { ScrollingModule } from '@angular/cdk/scrolling';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';

/**
 * Component to display a large list of batch jobs using virtual scrolling.
 */
@Component({
  selector: 'app-batch-jobs-list',
  imports: [CommonModule, ScrollingModule, AppBadgeComponent, DatePipe],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="jobs-list-container">
      <div class="list-header">
        <h3 class="m-0">Recent Batch Jobs</h3>
        <span class="text-muted">Total: {{ jobs().length }}</span>
      </div>

      @if (jobs().length === 0) {
        <div class="empty-state p-4 text-center text-muted">No jobs found.</div>
      } @else {
        <cdk-virtual-scroll-viewport itemSize="64" class="viewport">
          <div
            *cdkVirtualFor="let job of jobs(); trackBy: trackById"
            class="job-item"
          >
            <div class="job-info">
              <button
                type="button"
                class="text-bold text-decoration-none color-fg-default"
                (click)="select.emit(job.id)"
              >
                {{ job.id.substring(0, 8) }}...
              </button>
              <span class="text-muted text-small ml-2">{{ job.target }}</span>
            </div>

            <div class="job-meta">
              <span class="text-muted text-small mr-3">{{
                job.createdAt | date: 'short'
              }}</span>
              <app-badge [variant]="getStatusVariant(job.status)">
                {{ job.status }}
              </app-badge>
            </div>
          </div>
        </cdk-virtual-scroll-viewport>
      }
    </div>
  `,
  styles: `
    .jobs-list-container {
      border: 1px solid var(--color-border-default, #d0d7de);
      border-radius: 6px;
      overflow: hidden;
      background-color: var(--color-bg-default, #ffffff);
    }
    .list-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 16px;
      border-bottom: 1px solid var(--color-border-default, #d0d7de);
      background-color: var(--color-bg-subtle, #f6f8fa);
    }
    .m-0 {
      margin: 0;
    }
    .p-4 {
      padding: 24px;
    }
    .ml-2 {
      margin-left: 8px;
    }
    .mr-3 {
      margin-right: 16px;
    }
    .text-center {
      text-align: center;
    }
    .text-bold {
      font-weight: 600;
    }
    .text-small {
      font-size: 12px;
    }
    .text-decoration-none {
      text-decoration: none;
    }
    .color-fg-default {
      color: var(--color-fg-default, #24292f);
    }
    .color-fg-default {
      background: none;
      border: none;
      padding: 0;
      cursor: pointer;
      font: inherit;
    }
    .color-fg-default:hover {
      color: var(--color-accent-fg, #0969da);
      text-decoration: underline;
    }
    .text-muted {
      color: var(--color-fg-muted, #57606a);
    }

    .viewport {
      height: 600px;
      width: 100%;
    }

    .job-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 0 16px;
      height: 64px;
      border-bottom: 1px solid var(--color-border-subtle, #ebf0f4);
      box-sizing: border-box;
    }
    .job-item:hover {
      background-color: var(--color-bg-subtle, #f6f8fa);
    }
    .job-info,
    .job-meta {
      display: flex;
      align-items: center;
    }
  `,
})
export class BatchJobsListComponent {
  /** List of jobs to display */
  jobs = input<BatchJob[]>([]);

  /** Emitted when a job is selected */
  select = output<string>();

  /**
   * Track by ID function for virtual scroll.
   * @param index Item index
   * @param job Job item
   * @returns Job ID string
   */
  trackById(index: number, job: BatchJob): string {
    return job.id;
  }

  /**
   * Helper to map job status to badge variant.
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
