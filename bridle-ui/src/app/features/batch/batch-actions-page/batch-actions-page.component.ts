import { Component, inject, OnInit, viewChild, signal } from '@angular/core';
import { BatchService } from '../../../core/services/batch.service';
import { NotificationService } from '../../../core/services/notification.service';
import { JobsStore } from '../../../core/store/jobs.store';
import { JsonObject } from '../../../core/models/models';
import {
  AppTabsComponent,
  TabItem,
} from '../../../shared/ui/app-tabs/app-tabs.component';
import { BatchFixFormComponent } from '../batch-fix-form/batch-fix-form.component';
import { BatchRunFormComponent } from '../batch-run-form/batch-run-form.component';
import { BatchJobsListComponent } from '../batch-jobs-list/batch-jobs-list.component';
import { BatchJobDetailComponent } from '../batch-job-detail/batch-job-detail.component';

/**
 * Page component for managing batch operations and pipelines.
 */
@Component({
  selector: 'app-batch-actions-page',
  imports: [
    AppTabsComponent,
    BatchFixFormComponent,
    BatchRunFormComponent,
    BatchJobsListComponent,
    BatchJobDetailComponent,
  ],
  template: `
    <div class="batch-page-container">
      <div class="page-header">
        <h1 class="h2">Batch Actions & Pipelines</h1>
      </div>

      <div class="layout-grid">
        <div class="main-column">
          <app-tabs [tabs]="tabs" [(activeTab)]="activeTab">
            @if (activeTab() === 'fix') {
              <app-batch-fix-form
                (createJob)="onCreateFixJob($event)"
              ></app-batch-fix-form>
            } @else {
              <app-batch-run-form
                (runPipeline)="onRunPipeline($event)"
              ></app-batch-run-form>
            }
          </app-tabs>
        </div>

        <div class="sidebar-column">
          @if (activeJob()) {
            <div class="mb-4">
              <app-batch-job-detail
                [job]="activeJob()"
                (resume)="onResumeJob($event)"
                (close)="onCloseJobDetail()"
              ></app-batch-job-detail>
            </div>
          }

          <app-batch-jobs-list
            [jobs]="jobs()"
            (selectJob)="onSelectJob($event)"
          ></app-batch-jobs-list>
        </div>
      </div>
    </div>
  `,
  styles: `
    .batch-page-container {
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
      grid-template-columns: 1fr 480px;
      gap: 24px;
      align-items: start;
    }

    @media (max-width: 960px) {
      .layout-grid {
        grid-template-columns: 1fr;
      }
    }

    .mb-4 {
      margin-bottom: 24px;
    }
  `,
})
export class BatchActionsPageComponent implements OnInit {
  /** Injected BatchService */
  private batchService = inject(BatchService);
  /** Injected JobsStore */
  private jobsStore = inject(JobsStore);
  /** Injected NotificationService */
  private notificationService = inject(NotificationService);

  /** Signal query for BatchFixFormComponent */
  readonly fixFormComponent = viewChild(BatchFixFormComponent);
  /** Signal query for BatchRunFormComponent */
  readonly runFormComponent = viewChild(BatchRunFormComponent);

  /** Signal of all jobs */
  jobs = this.jobsStore.jobs;
  /** Signal of currently active job */
  activeJob = this.jobsStore.activeJob;

  /** Available tabs */
  tabs: TabItem[] = [
    { id: 'fix', label: 'Batch Fix' },
    { id: 'pipeline', label: 'Run Pipeline' },
  ];

  /** Active tab ID */
  activeTab = signal('fix');

  /** Initializes the component and loads jobs */
  ngOnInit(): void {
    this.jobsStore.loadJobs();
  }

  /**
   * Handles creating a batch fix job.
   * @param data Form data
   * @param data.orgId Target organization ID
   * @param data.title Issue title
   * @param data.description Issue description
   * @param data.pattern Target regex pattern
   * @param data.tools Tools to run
   * @param data.args Tool arguments
   * @param data.safety_mode Whether to run in safety mode
   * @param data.max_repos Maximum number of repositories
   * @param data.max_prs_per_hour Maximum PRs per hour
   */
  onCreateFixJob(data: {
    orgId: string;
    title: string;
    description: string;
    pattern: string;
    tools: string[];
    args: JsonObject;
    safety_mode: boolean;
    max_repos?: number;
    max_prs_per_hour?: number;
  }): void {
    this.fixFormComponent()?.setSubmitting(true);
    this.batchService
      .createBatchFix(
        data.orgId,
        data.title,
        data.description,
        data.pattern,
        data.tools,
        data.args,
        data.safety_mode,
        data.max_repos,
        data.max_prs_per_hour,
      )
      .subscribe({
        next: () => {
          this.fixFormComponent()?.setSubmitting(false);
          this.notificationService.success('Batch fix job created');
          this.jobsStore.loadJobs();
        },
        error: () => {
          this.fixFormComponent()?.setSubmitting(false);
          this.notificationService.error('Failed to create batch fix job');
        },
      });
  }

  /**
   * Handles running a pipeline.
   * @param data Form data
   * @param data.configYaml Pipeline YAML configuration
   * @param data.safety_mode Whether to run in safety mode
   * @param data.max_repos Maximum number of repositories
   * @param data.max_prs_per_hour Maximum PRs per hour
   */
  onRunPipeline(data: {
    configYaml: string;
    safety_mode: boolean;
    max_repos?: number;
    max_prs_per_hour?: number;
  }): void {
    this.runFormComponent()?.setSubmitting(true);
    this.batchService
      .runPipeline(
        data.configYaml,
        data.safety_mode,
        data.max_repos,
        data.max_prs_per_hour,
      )
      .subscribe({
        next: () => {
          this.runFormComponent()?.setSubmitting(false);
          this.notificationService.success('Pipeline started');
          this.jobsStore.loadJobs();
        },
        error: () => {
          this.runFormComponent()?.setSubmitting(false);
          this.notificationService.error('Failed to start pipeline');
        },
      });
  }

  /**
   * Handles selecting a job from the list.
   * @param jobId The selected job ID
   */
  onSelectJob(jobId: string): void {
    this.jobsStore.setActiveJob(jobId);
  }

  /**
   * Handles resuming a job.
   * @param jobId The job ID to resume
   */
  onResumeJob(jobId: string): void {
    this.batchService.resumeJob(jobId).subscribe({
      next: () => {
        this.notificationService.success(`Job ${jobId} resumed`);
        this.jobsStore.loadJobs();
      },
      error: () => this.notificationService.error('Failed to resume job'),
    });
  }

  /**
   * Closes the job detail view.
   */
  onCloseJobDetail(): void {
    this.jobsStore.setActiveJob(null);
  }
}
