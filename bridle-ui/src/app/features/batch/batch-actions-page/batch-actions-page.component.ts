import { Component, ChangeDetectionStrategy, inject, OnInit, ViewChild, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BatchService } from '../../../core/services/batch.service';
import { NotificationService } from '../../../core/services/notification.service';
import { AppTabsComponent, TabItem } from '../../../shared/ui/app-tabs/app-tabs.component';
import { BatchFixFormComponent } from '../batch-fix-form/batch-fix-form.component';
import { BatchRunFormComponent } from '../batch-run-form/batch-run-form.component';
import { BatchJobsListComponent } from '../batch-jobs-list/batch-jobs-list.component';
import { BatchJobDetailComponent } from '../batch-job-detail/batch-job-detail.component';
import { BatchJob } from '../../../core/models/models';
import { JobsStore } from '../../../core/store/jobs.store';

/**
 * Main page component for batch operations.
 */
@Component({
  selector: 'app-batch-actions-page',
  imports: [
    CommonModule, 
    AppTabsComponent, 
    BatchFixFormComponent, 
    BatchRunFormComponent, 
    BatchJobsListComponent, 
    BatchJobDetailComponent
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="container-lg">
      <div class="mb-4">
        <h2 class="mb-2">Batch Actions</h2>
        <p class="text-muted">Queue multi-repository operations, fixes, and pipeline runs.</p>
      </div>

      <div class="layout-grid">
        <div class="left-col">
          <div class="mb-4">
            <app-tabs [tabs]="tabs" [activeTabId]="activeTabId()" (tabChange)="onTabChange($event)"></app-tabs>
          </div>

          <div class="form-container mb-4">
            @if (activeTabId() === 'fix') {
              <app-batch-fix-form (createJob)="onCreateBatchFix($event)"></app-batch-fix-form>
            } @else {
              <app-batch-run-form (runPipeline)="onRunPipeline($event)"></app-batch-run-form>
            }
          </div>
        </div>

        <div class="right-col">
          @if (jobsStore.activeJob()) {
            <app-batch-job-detail 
              [job]="jobsStore.activeJob()!" 
              (close)="onCloseJobDetail()"
              (resume)="onResumeJob($event)">
            </app-batch-job-detail>
          } @else {
            @if (jobsStore.isLoading()) {
              <div class="p-4 text-center text-muted">Loading jobs...</div>
            } @else {
              <app-batch-jobs-list 
                [jobs]="jobsStore.filteredJobs()" 
                (select)="onSelectJob($event)">
              </app-batch-jobs-list>
            }
          }
        </div>
      </div>
    </div>
  `,
  styles: `
    .mb-2 { margin-bottom: 8px; }
    .mb-4 { margin-bottom: 24px; }
    .p-4 { padding: 24px; }
    .text-center { text-align: center; }
    .text-muted { color: var(--color-fg-muted); }
    
    .layout-grid {
      display: grid;
      grid-template-columns: 1fr 1.5fr;
      gap: 24px;
      align-items: start;
    }
    
    @media (max-width: 900px) {
      .layout-grid {
        grid-template-columns: 1fr;
      }
    }
  `
})
export class BatchActionsPageComponent implements OnInit {
  /** Jobs store instance */
  jobsStore = inject(JobsStore);
  /** Batch service instance */
  private batchService = inject(BatchService);
  /** Notification service instance */
  private notificationService = inject(NotificationService);

  /** Reference to fix form */
  @ViewChild(BatchFixFormComponent) fixFormComponent!: BatchFixFormComponent;
  /** Reference to run form */
  @ViewChild(BatchRunFormComponent) runFormComponent!: BatchRunFormComponent;

  /** Tabs for the operations */
  tabs: TabItem[] = [
    { id: 'fix', label: 'Batch Fix' },
    { id: 'run', label: 'Pipeline Run' }
  ];

  /** Active tab ID */
  activeTabId = signal<string>('fix');

  /** Initialize and load jobs */
  ngOnInit(): void {
    // Automatically trigger loading jobs through the store RxMethod
    this.jobsStore.loadJobs();
  }

  /**
   * Handles tab changes.
   * @param id Selected tab ID
   */
  onTabChange(id: string): void {
    this.activeTabId.set(id);
  }

  /**
   * Handles creating a new batch fix job.
   */
  onCreateBatchFix(payload: { target: string; title: string; description: string; pattern: string; tools: string[]; args: Record<string, unknown>; safety_mode: boolean; max_repos?: number; max_prs_per_hour?: number }): void {
    this.fixFormComponent?.setSubmitting(true);
    this.batchService.createBatchFix(payload.target, payload.title, payload.description, payload.pattern, payload.tools, payload.args, payload.safety_mode, payload.max_repos, payload.max_prs_per_hour).subscribe({
      next: (job: BatchJob) => {
        this.fixFormComponent?.setSubmitting(false);
        this.notificationService.success(`Job ${job.id.toString()} queued successfully`);
        this.jobsStore.addJob(job);
        this.onSelectJob(job.id);
      },
      error: () => {
        this.fixFormComponent?.setSubmitting(false);
        this.notificationService.error('Failed to queue batch fix');
      }
    });
  }

  /**
   * Handles running a pipeline.
   */
  onRunPipeline(payload: { config: string; safety_mode: boolean; max_repos?: number; max_prs_per_hour?: number }): void {
    this.runFormComponent?.setSubmitting(true);
    this.batchService.runPipeline(payload.config, payload.safety_mode, payload.max_repos, payload.max_prs_per_hour).subscribe({
      next: (job: BatchJob) => {
        this.runFormComponent?.setSubmitting(false);
        this.notificationService.success(`Pipeline job ${job.id.toString()} started`);
        this.jobsStore.addJob(job);
        this.onSelectJob(job.id);
      },
      error: () => {
        this.runFormComponent?.setSubmitting(false);
        this.notificationService.error('Failed to start pipeline');
      }
    });
  }

  /**
   * Handles selecting a job to view details.
   * @param id Job ID
   */
  onSelectJob(id: string): void {
    this.jobsStore.setActiveJob(id);
  }

  /**
   * Resumes an interrupted job.
   * @param id Job ID
   */
  onResumeJob(id: string): void {
    this.batchService.resumeJob(id).subscribe({
      next: () => {
        this.notificationService.success('Job resumed successfully');
        this.jobsStore.loadJobs();
      },
      error: () => this.notificationService.error('Failed to resume job')
    });
  }

  /**
   * Closes the job detail view.
   */
  onCloseJobDetail(): void {
    this.jobsStore.setActiveJob(null);
  }
}
