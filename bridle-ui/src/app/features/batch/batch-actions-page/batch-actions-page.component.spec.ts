import { ComponentFixture, TestBed } from '@angular/core/testing';
import { WritableSignal, signal } from '@angular/core';
import { of, throwError } from 'rxjs';
import { BatchActionsPageComponent } from './batch-actions-page.component';
import { BatchService } from '../../../core/services/batch.service';
import { NotificationService } from '../../../core/services/notification.service';
import { JobsStore } from '../../../core/store/jobs.store';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';

interface MockNotificationService {
  success: jasmine.Spy;
  error: jasmine.Spy;
  info: jasmine.Spy;
}

interface MockBatchService {
  createBatchFix: jasmine.Spy;
  runPipeline: jasmine.Spy;
  resumeJob: jasmine.Spy;
}

interface MockJobsStore {
  jobs: WritableSignal<BatchJob[]>;
  activeJob: WritableSignal<BatchJob | null>;
  isLoading: WritableSignal<boolean>;
  loadJobs: jasmine.Spy;
  setActiveJob: jasmine.Spy;
}

describe('BatchActionsPageComponent', () => {
  let component: BatchActionsPageComponent;
  let fixture: ComponentFixture<BatchActionsPageComponent>;
  let mockBatchService: MockBatchService;
  let mockNotificationService: MockNotificationService;
  let mockJobsStore: MockJobsStore;

  beforeEach(async () => {
    mockBatchService = {
      createBatchFix: jasmine
        .createSpy('createBatchFix')
        .and.returnValue(of({ id: '123' })),
      runPipeline: jasmine
        .createSpy('runPipeline')
        .and.returnValue(of({ id: '456' })),
      resumeJob: jasmine.createSpy('resumeJob').and.returnValue(of({})),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      error: jasmine.createSpy('error'),
      info: jasmine.createSpy('info'),
    };

    mockJobsStore = {
      jobs: signal([]),
      activeJob: signal({
        id: '123',
        target: 'org',
        status: BatchJobStatus.INTERRUPTED,
        createdAt: '2026-01-01T00:00:00Z',
      }),
      isLoading: signal(false),
      loadJobs: jasmine.createSpy('loadJobs'),
      setActiveJob: jasmine.createSpy('setActiveJob'),
    };

    await TestBed.configureTestingModule({
      imports: [BatchActionsPageComponent],
      providers: [
        { provide: BatchService, useValue: mockBatchService },
        { provide: NotificationService, useValue: mockNotificationService },
        { provide: JobsStore, useValue: mockJobsStore },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(BatchActionsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should load jobs on init', () => {
    expect(mockJobsStore.loadJobs).toHaveBeenCalled();
  });

  it('should change tabs', () => {
    expect(component.activeTab()).toBe('fix');
    component.activeTab.set('pipeline');
    fixture.detectChanges();
    expect(component.activeTab()).toBe('pipeline');
  });

  it('should create batch fix job', () => {
    component.onCreateFixJob({
      orgId: 'org',
      title: 't',
      description: 'd',
      pattern: 'p',
      tools: ['eslint'],
      args: {},
      safety_mode: true,
      max_repos: 10,
      max_prs_per_hour: 5,
    });
    expect(mockBatchService.createBatchFix).toHaveBeenCalledWith(
      'org',
      't',
      'd',
      'p',
      ['eslint'],
      {},
      true,
      10,
      5,
    );
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Batch fix job created',
    );
    expect(mockJobsStore.loadJobs).toHaveBeenCalled();
  });

  it('should handle create batch fix error', () => {
    mockBatchService.createBatchFix.and.returnValue(
      throwError(() => new Error('err')),
    );

    component.onCreateFixJob({
      orgId: 'org',
      title: 't',
      description: 'd',
      pattern: 'p',
      tools: [],
      args: {},
      safety_mode: true,
    });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to create batch fix job',
    );
  });

  it('should run pipeline', () => {
    component.activeTab.set('pipeline');
    fixture.detectChanges();

    component.onRunPipeline({
      configYaml: 'yaml',
      safety_mode: true,
      max_repos: 10,
      max_prs_per_hour: 5,
    });
    expect(mockBatchService.runPipeline).toHaveBeenCalledWith(
      'yaml',
      true,
      10,
      5,
    );
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Pipeline started',
    );
    expect(mockJobsStore.loadJobs).toHaveBeenCalled();
  });

  it('should handle run pipeline error', () => {
    component.activeTab.set('pipeline');
    fixture.detectChanges();

    mockBatchService.runPipeline.and.returnValue(
      throwError(() => new Error('err')),
    );

    component.onRunPipeline({
      configYaml: 'yaml',
      safety_mode: true,
    });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to start pipeline',
    );
  });

  it('should select job', () => {
    component.onSelectJob('123');
    expect(mockJobsStore.setActiveJob).toHaveBeenCalledWith('123');
  });

  it('should resume job', () => {
    component.onResumeJob('123');
    expect(mockBatchService.resumeJob).toHaveBeenCalledWith('123');
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Job 123 resumed',
    );
    expect(mockJobsStore.loadJobs).toHaveBeenCalled();
  });

  it('should handle resume job error', () => {
    mockBatchService.resumeJob.and.returnValue(
      throwError(() => new Error('err')),
    );
    component.onResumeJob('123');
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to resume job',
    );
  });

  it('should close job detail', () => {
    component.onCloseJobDetail();
    expect(mockJobsStore.setActiveJob).toHaveBeenCalledWith(null);
  });
});
