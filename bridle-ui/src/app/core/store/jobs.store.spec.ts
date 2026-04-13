import { TestBed } from '@angular/core/testing';
import { JobsStore } from './jobs.store';
import { BatchService } from '../services/batch.service';
import { NotificationService } from '../services/notification.service';
import { of, throwError } from 'rxjs';
import { BatchJob, BatchJobStatus } from '../models/models';

describe('JobsStore', () => {
  let store: InstanceType<typeof JobsStore>;
  let batchServiceSpy: jasmine.SpyObj<BatchService>;
  let notificationServiceSpy: jasmine.SpyObj<NotificationService>;

  beforeEach(() => {
    const batchSpy = jasmine.createSpyObj('BatchService', ['loadJobs']);
    const notifSpy = jasmine.createSpyObj('NotificationService', [
      'error',
      'success',
    ]);

    TestBed.configureTestingModule({
      providers: [
        { provide: BatchService, useValue: batchSpy },
        { provide: NotificationService, useValue: notifSpy },
      ],
    });

    batchServiceSpy = TestBed.inject(
      BatchService,
    ) as jasmine.SpyObj<BatchService>;
    notificationServiceSpy = TestBed.inject(
      NotificationService,
    ) as jasmine.SpyObj<NotificationService>;
    store = TestBed.inject(JobsStore);
  });

  it('should be created with initial state', () => {
    expect(store.jobs()).toEqual([]);
    expect(store.activeJobId()).toBeNull();
    expect(store.isLoading()).toBeFalse();
    expect(store.error()).toBeNull();
    expect(store.filterStatus()).toBe('ALL');
    expect(store.pageIndex()).toBe(0);
    expect(store.pageSize()).toBe(100);
  });

  it('should set filter status', () => {
    store.setFilter('RUNNING' as BatchJobStatus);
    expect(store.filterStatus()).toBe('RUNNING');
  });

  it('should set active job ID', () => {
    store.setActiveJob('job-123');
    expect(store.activeJobId()).toBe('job-123');
  });

  it('should add a job', () => {
    const job = {
      id: 'job-1',
      name: 'Job 1',
      status: 'PENDING',
    } as unknown as BatchJob;
    store.addJob(job);
    expect(store.jobs()).toEqual([job]);
  });

  it('should filter jobs correctly', () => {
    const job1 = { id: '1', status: 'PENDING' } as unknown as BatchJob;
    const job2 = { id: '2', status: 'RUNNING' } as unknown as BatchJob;
    store.addJob(job1);
    store.addJob(job2);

    expect(store.filteredJobs()).toEqual([job2, job1]);

    store.setFilter(BatchJobStatus.PENDING);
    expect(store.filteredJobs()).toEqual([job1]);

    store.setFilter(BatchJobStatus.RUNNING);
    expect(store.filteredJobs()).toEqual([job2]);

    store.setFilter('ALL');
    expect(store.filteredJobs()).toEqual([job2, job1]);
  });

  it('should compute active job', () => {
    const job1 = { id: '1', name: 'J1' } as unknown as BatchJob;
    store.addJob(job1);
    expect(store.activeJob()).toBeNull();

    store.setActiveJob('1');
    expect(store.activeJob()).toEqual(job1);
  });

  it('should compute total jobs', () => {
    expect(store.totalJobs()).toBe(0);
    store.addJob({} as unknown as BatchJob);
    expect(store.totalJobs()).toBe(1);
  });

  it('should load jobs successfully', () => {
    const jobs = [{ id: '1' } as unknown as BatchJob];
    batchServiceSpy.loadJobs.and.returnValue(of(jobs));

    store.loadJobs();

    expect(batchServiceSpy.loadJobs).toHaveBeenCalled();
    expect(store.jobs()).toEqual(jobs);
    expect(store.isLoading()).toBeFalse();
    expect(store.error()).toBeNull();
  });

  it('should handle error when loading jobs', () => {
    batchServiceSpy.loadJobs.and.returnValue(
      throwError(() => new Error('Load failed')),
    );

    store.loadJobs();

    expect(store.error()).toBe('Load failed');
    expect(store.isLoading()).toBeFalse();
    expect(notificationServiceSpy.error).toHaveBeenCalledWith('Load failed');
    expect(store.jobs()).toEqual([]);
  });

  it('should handle error without message when loading jobs', () => {
    batchServiceSpy.loadJobs.and.returnValue(throwError(() => ({})));

    store.loadJobs();

    expect(store.error()).toBe('Failed to load jobs');
    expect(store.isLoading()).toBeFalse();
    expect(notificationServiceSpy.error).toHaveBeenCalledWith(
      'Failed to load jobs',
    );
    expect(store.jobs()).toEqual([]);
  });
});
