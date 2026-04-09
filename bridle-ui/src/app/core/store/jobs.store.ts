import { computed, inject } from '@angular/core';
import { signalStore, withState, withMethods, withComputed, patchState } from '@ngrx/signals';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { pipe, switchMap, tap, map, catchError, of } from 'rxjs';
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

/** Initial state for jobs store */
const initialState: JobsState = {
  jobs: [],
  activeJobId: null,
  isLoading: false,
  error: null,
  filterStatus: 'ALL',
  pageIndex: 0,
  pageSize: 100,
};

/** Signal store for managing jobs */
export const JobsStore = signalStore(
  { providedIn: 'root' },
  withState(initialState),
  withComputed((store) => ({
    filteredJobs: computed(() => {
      const jobs = store.jobs();
      const status = store.filterStatus();
      return status === 'ALL' ? jobs : jobs.filter((job) => job.status === status);
    }),
    activeJob: computed(() => {
      const id = store.activeJobId();
      return store.jobs().find((job) => job.id === id) || null;
    }),
    totalJobs: computed(() => store.jobs().length),
  })),
  withMethods((store, batchService = inject(BatchService), notificationService = inject(NotificationService)) => ({
    setFilter(status: BatchJobStatus | 'ALL') {
      patchState(store, { filterStatus: status });
    },
    setActiveJob(id: string | null) {
      patchState(store, { activeJobId: id });
    },
    addJob(job: BatchJob) {
      patchState(store, { jobs: [job, ...store.jobs()] });
    },
    loadJobs: rxMethod<void>(
      pipe(
        tap(() => patchState(store, { isLoading: true, error: null })),
        switchMap(() => {
          return batchService.loadJobs().pipe(
            tap((jobs) => patchState(store, { jobs, isLoading: false })),
            catchError((err) => {
              const errorMsg = err.message || 'Failed to load jobs';
              patchState(store, { error: errorMsg, isLoading: false });
              notificationService.error(errorMsg);
              return of([]);
            })
          );
        })
      )
    ),
  }))
);
