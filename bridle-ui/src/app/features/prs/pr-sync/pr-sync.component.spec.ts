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
  jobs: WritableSignal<BatchJob[]>;
  activeJob: WritableSignal<BatchJob | null>;
  isLoading: WritableSignal<boolean>;
  loadJobs: jasmine.Spy;
  addJob: jasmine.Spy;
  setActiveJob: jasmine.Spy;
}

import { ComponentFixture, TestBed } from '@angular/core/testing';
import { PrSyncComponent } from './pr-sync.component';
import { OrgService } from '../../../core/services/org.service';
import { PrService } from '../../../core/services/pr.service';
import { NotificationService } from '../../../core/services/notification.service';
import { of, throwError } from 'rxjs';
import { signal } from '@angular/core';

describe('PrSyncComponent', () => {
  let component: PrSyncComponent;
  let fixture: ComponentFixture<PrSyncComponent>;
  let mockOrgService: MockOrgService;
  let mockPrService: MockPrService;
  let mockNotificationService: MockNotificationService;

  beforeEach(async () => {
    mockOrgService = {
      orgs: signal([{ id: '1', name: 'Org 1', provider: 'github' }]),
      repos: signal([]),
      isLoading: signal(false),
      loadOrgs: jasmine.createSpy('loadOrgs').and.returnValue(of([])),
      ingestOrg: jasmine.createSpy('ingestOrg'),
      loadRepos: jasmine.createSpy('loadRepos'),
    };

    mockPrService = {
      prs: signal([
        { id: 'pr1', title: 'Test PR', repoId: 'repo1', status: 'LOCAL' },
      ]),
      isSyncing: signal(false),
      loadPrs: jasmine.createSpy('loadPrs').and.returnValue(of([])),
      syncPrs: jasmine
        .createSpy('syncPrs')
        .and.returnValue(of({ syncedCount: 1 })),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      error: jasmine.createSpy('error'),
    } as object as MockNotificationService;

    await TestBed.configureTestingModule({
      imports: [PrSyncComponent],
      providers: [
        { provide: OrgService, useValue: mockOrgService },
        { provide: PrService, useValue: mockPrService },
        { provide: NotificationService, useValue: mockNotificationService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(PrSyncComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should initialize org options', () => {
    const options = component.orgOptions();
    expect(options.length).toBe(2);
    expect(options[0].value).toBe('');
    expect(options[1].value).toBe('1');
  });

  it('should load orgs on init if empty', () => {
    mockOrgService.orgs.set([]);
    component.ngOnInit();
    expect(mockOrgService.loadOrgs).toHaveBeenCalled();
  });

  it('should load PRs when org is selected', () => {
    component.form.get('orgId')?.setValue('1');
    expect(mockPrService.loadPrs).toHaveBeenCalledWith('1');
  });

  it('should handle load PRs error', () => {
    mockPrService.loadPrs.and.returnValue(
      throwError(() => new Error('Failed to load PRs')),
    );
    component.form.get('orgId')?.setValue('2');
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to load PRs',
    );
  });

  it('should sync PRs and show success', () => {
    component.form.setValue({
      orgId: '1',
      maxRate: 5,
    });

    component.onSync();

    expect(mockPrService.syncPrs).toHaveBeenCalledWith('1', 5);
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Successfully synced 1 PRs.',
    );
    expect(mockPrService.loadPrs).toHaveBeenCalledWith('1'); // Should reload after sync
  });

  it('should handle sync PRs error', () => {
    mockPrService.syncPrs.and.returnValue(
      throwError(() => new Error('Failed to sync PRs')),
    );

    component.form.setValue({
      orgId: '1',
      maxRate: 5,
    });

    component.onSync();

    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to sync PRs',
    );
  });

  it('should track by id', () => {
    expect(
      component.trackById(0, {
        id: 'pr123',
        title: '',
        repoId: '',
        status: 'LOCAL',
      }),
    ).toBe('pr123');
  });

  it('should not sync if form is invalid', () => {
    mockPrService.syncPrs.calls.reset();
    component.form.setValue({
      orgId: '',
      maxRate: 5,
    });
    component.onSync();
    expect(mockPrService.syncPrs).not.toHaveBeenCalled();
  });
});
