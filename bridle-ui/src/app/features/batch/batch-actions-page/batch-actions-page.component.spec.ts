import { WritableSignal } from '@angular/core';
import {
  Organization,
  Repository,
  PullRequest,
  BatchJob,
  SystemHealth,
} from '../../../core/models/models';

interface MockOrgService {
  orgs: WritableSignal<Organization[]>;
  repos: WritableSignal<Repository[]>;
  isLoading: WritableSignal<boolean>;
  loadOrgs: jasmine.Spy;
  ingestOrg: jasmine.Spy;
  loadRepos: jasmine.Spy;
}

interface MockPrService {
  prs: WritableSignal<PullRequest[]>;
  isSyncing: WritableSignal<boolean>;
  loadPrs: jasmine.Spy;
  syncPrs: jasmine.Spy;
}

interface MockNotificationService {
  success: jasmine.Spy;
  error: jasmine.Spy;
  info: jasmine.Spy;
}

interface MockSystemStateService {
  health: WritableSignal<SystemHealth>;
  isLoading: WritableSignal<boolean>;
  checkHealth: jasmine.Spy;
}

interface MockBatchService {
  createBatchFix: jasmine.Spy;
  runPipeline: jasmine.Spy;
  resumeJob: jasmine.Spy;
}

interface MockApiService {
  post: jasmine.Spy;
}

interface MockLocalOpService {
  audit: jasmine.Spy;
  fix: jasmine.Spy;
  clearResult: jasmine.Spy;
}

interface MockJobsStore {
  filteredJobs: WritableSignal<BatchJob[]>;
  jobs: WritableSignal<BatchJob[]>;
  activeJob: WritableSignal<BatchJob | null>;
  isLoading: WritableSignal<boolean>;
  loadJobs: jasmine.Spy;
  addJob: jasmine.Spy;
  setActiveJob: jasmine.Spy;
}

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
    } as object as MockNotificationService;

    mockJobsStore = {
      jobs: signal([]),
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
      form: {} as object as import('@angular/forms').FormGroup,
      fb: {} as object as import('@angular/forms').FormBuilder,
      createBatchFix:
        {} as object as import('@angular/core').EventEmitter<object>,
      isSubmitting:
        {} as object as import('@angular/core').InputSignal<boolean>,
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as object as import('../batch-fix-form/batch-fix-form.component').BatchFixFormComponent;
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
      form: {} as object as import('@angular/forms').FormGroup,
      fb: {} as object as import('@angular/forms').FormBuilder,
      createBatchFix:
        {} as object as import('@angular/core').EventEmitter<object>,
      isSubmitting:
        {} as object as import('@angular/core').InputSignal<boolean>,
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as object as import('../batch-fix-form/batch-fix-form.component').BatchFixFormComponent;    mockBatchService.createBatchFix.and.returnValue(
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
      form: {} as object as import('@angular/forms').FormGroup,
      fb: {} as object as import('@angular/forms').FormBuilder,
      runPipeline: {} as object as import('@angular/core').EventEmitter<object>,
      isSubmitting:
        {} as object as import('@angular/core').InputSignal<boolean>,
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as object as import('../batch-run-form/batch-run-form.component').BatchRunFormComponent;
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
      form: {} as object as import('@angular/forms').FormGroup,
      fb: {} as object as import('@angular/forms').FormBuilder,
      runPipeline: {} as object as import('@angular/core').EventEmitter<object>,
      isSubmitting:
        {} as object as import('@angular/core').InputSignal<boolean>,
      setSubmitting: jasmine.createSpy('setSubmitting'),
    } as object as import('../batch-run-form/batch-run-form.component').BatchRunFormComponent;    mockBatchService.runPipeline.and.returnValue(
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
