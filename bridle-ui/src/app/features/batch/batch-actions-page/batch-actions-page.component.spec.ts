import { ComponentFixture, TestBed } from '@angular/core/testing';
import { BatchActionsPageComponent } from './batch-actions-page.component';
import { BatchService } from '../../../core/services/batch.service';
import { NotificationService } from '../../../core/services/notification.service';
import { JobsStore } from '../../../core/store/jobs.store';
import { of, throwError } from 'rxjs';
import { signal } from '@angular/core';

describe('BatchActionsPageComponent', () => {
  let component: BatchActionsPageComponent;
  let fixture: ComponentFixture<BatchActionsPageComponent>;
  let mockBatchService: any;
  let mockNotificationService: any;
  let mockJobsStore: Record<string, any>;

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
    };

    mockJobsStore = {
      activeJob: signal(null),
      isLoading: signal(false),
      filteredJobs: signal([]),
      loadJobs: jasmine.createSpy('loadJobs'),
      setActiveJob: jasmine.createSpy('setActiveJob'),
      addJob: jasmine.createSpy('addJob'),
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
    expect(mockJobsStore['loadJobs']).toHaveBeenCalled();
  });

  it('should change tabs', () => {
    expect(component.activeTabId()).toBe('fix');
    component.onTabChange('run');
    expect(component.activeTabId()).toBe('run');
  });

  it('should create batch fix', () => {
    component.fixFormComponent = {
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as any;

    component.onCreateBatchFix({
      target: 'org',
      title: 't',
      description: 'd',
      pattern: 'p',
      tools: [],
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
      [],
      {},
      true,
      10,
      5,
    );
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Job 123 queued successfully',
    );
    expect(mockJobsStore['addJob']).toHaveBeenCalledWith({ id: '123' });
    expect(mockJobsStore['setActiveJob']).toHaveBeenCalledWith('123');
  });

  it('should handle create batch fix error', () => {
    component.fixFormComponent = {
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as any;
    mockBatchService.createBatchFix.and.returnValue(
      throwError(() => new Error('err')),
    );

    component.onCreateBatchFix({
      target: 'org',
      title: 't',
      description: 'd',
      pattern: 'p',
      tools: [],
      args: {},
      safety_mode: true,
      max_repos: 10,
      max_prs_per_hour: 5,
    });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to queue batch fix',
    );
  });

  it('should run pipeline', () => {
    component.runFormComponent = {
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as any;

    component.onRunPipeline({
      config: 'yaml',
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
      'Pipeline job 456 started',
    );
    expect(mockJobsStore['addJob']).toHaveBeenCalledWith({ id: '456' });
    expect(mockJobsStore['setActiveJob']).toHaveBeenCalledWith('456');
  });

  it('should handle run pipeline error', () => {
    component.runFormComponent = {
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as any;
    mockBatchService.runPipeline.and.returnValue(
      throwError(() => new Error('err')),
    );

    component.onRunPipeline({
      config: 'yaml',
      safety_mode: true,
      max_repos: 10,
      max_prs_per_hour: 5,
    });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to start pipeline',
    );
  });

  it('should select job', () => {
    component.onSelectJob('123');
    expect(mockJobsStore['setActiveJob']).toHaveBeenCalledWith('123');
  });

  it('should resume job', () => {
    component.onResumeJob('123');
    expect(mockBatchService.resumeJob).toHaveBeenCalledWith('123');
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Job resumed successfully',
    );
    expect(mockJobsStore['loadJobs']).toHaveBeenCalled();
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
    expect(mockJobsStore['setActiveJob']).toHaveBeenCalledWith(null);
  });
});
