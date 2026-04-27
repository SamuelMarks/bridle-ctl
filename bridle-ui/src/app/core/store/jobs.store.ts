import { computed, inject, Injectable, signal } from '@angular/core';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { Subject, switchMap, tap, catchError, of } from 'rxjs';
import { BatchJob, BatchJobStatus } from '../models/models';
import { BatchService } from '../services/batch.service';
import { NotificationService } from '../services/notification.service';

/** Interface representing the state of jobs */
export interface JobsState {
  /** List of jobs */
  jobs: BatchJob[];
  /** Active job ID */
  activeJobId: string | null;
  /** Loading state */
  isLoading: boolean;
  /** Error message */
  error: string | null;
  /** Filter status */
  filterStatus: BatchJobStatus | 'ALL';
  /** Page index */
  pageIndex: number; // for hypothetical pagination
  /** Page size */
  pageSize: number;
}

/** Store for managing jobs */
@Injectable({ providedIn: 'root' })
export class JobsStore {
  /** BatchService instance */
  private batchService = inject(BatchService);
  /** NotificationService instance */
  private notificationService = inject(NotificationService);

  /** Internal signal holding the state */
  private state = signal<JobsState>({
    jobs: [],
    activeJobId: null,
    isLoading: false,
    error: null,
    filterStatus: 'ALL',
    pageIndex: 0,
    pageSize: 100,
  });

  /** Computed list of jobs */
  readonly jobs = computed(() => this.state().jobs);
  /** Computed active job ID */
  readonly activeJobId = computed(() => this.state().activeJobId);
  /** Computed loading state */
  readonly isLoading = computed(() => this.state().isLoading);
  /** Computed error state */
  readonly error = computed(() => this.state().error);
  /** Computed filter status */
  readonly filterStatus = computed(() => this.state().filterStatus);
  /** Computed page index */
  readonly pageIndex = computed(() => this.state().pageIndex);
  /** Computed page size */
  readonly pageSize = computed(() => this.state().pageSize);

  /** Computed list of filtered jobs based on filter status */
  readonly filteredJobs = computed(() => {
    const jobs = this.jobs();
    const status = this.filterStatus();
    return status === 'ALL'
      ? jobs
      : jobs.filter((job) => job.status === status);
  });

  /** Computed active job object */
  readonly activeJob = computed(() => {
    const id = this.activeJobId();
    return this.jobs().find((job) => job.id === id) || null;
  });

  /** Computed total jobs count */
  readonly totalJobs = computed(() => this.jobs().length);

  /** Subject to trigger loading jobs */
  private loadJobsSubject = new Subject<void>();

  /** Constructor initializes the stream for loading jobs */
  constructor() {
    this.loadJobsSubject
      .pipe(
        takeUntilDestroyed(),
        tap(() => this.patchState({ isLoading: true, error: null })),
        switchMap(() => {
          return this.batchService.loadJobs().pipe(
            tap((jobs) => this.patchState({ jobs, isLoading: false })),
            catchError((err) => {
              const errorMsg = err.message || 'Failed to load jobs';
              this.patchState({ error: errorMsg, isLoading: false });
              this.notificationService.error(errorMsg);
              return of([]);
            }),
          );
        }),
      )
      .subscribe();
  }

  /**
   * Updates the filter status
   * @param status The new status to filter by
   */
  setFilter(status: BatchJobStatus | 'ALL') {
    this.patchState({ filterStatus: status });
  }

  /**
   * Sets the active job ID
   * @param id The job ID to set as active
   */
  setActiveJob(id: string | null) {
    this.patchState({ activeJobId: id });
  }

  /**
   * Adds a new job to the list
   * @param job The new BatchJob
   */
  addJob(job: BatchJob) {
    this.patchState({ jobs: [job, ...this.jobs()] });
  }

  /**
   * Triggers loading jobs from the service
   */
  loadJobs() {
    this.loadJobsSubject.next();
  }

  /**
   * Partially updates the internal state
   * @param partialState The state changes to apply
   */
  private patchState(partialState: Partial<JobsState>) {
    this.state.update((state) => ({ ...state, ...partialState }));
  }
}
