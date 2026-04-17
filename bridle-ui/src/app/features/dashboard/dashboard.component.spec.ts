import { WritableSignal } from '@angular/core';
import {
  Organization,
  Repository,
  PullRequest,
  BatchJob,
  SystemHealth,
} from '../../core/models/models';

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
import { DashboardComponent } from './dashboard.component';
import { SystemStateService } from '../../core/services/system-state.service';
import { of } from 'rxjs';
import { signal } from '@angular/core';
import { By } from '@angular/platform-browser';

describe('DashboardComponent', () => {
  let component: DashboardComponent;
  let fixture: ComponentFixture<DashboardComponent>;
  let mockSystemStateService: MockSystemStateService;

  beforeEach(async () => {
    mockSystemStateService = {
      health: signal({ rest: 'UP', rpc: 'DOWN', agent: 'UP' }),
      isLoading: signal(false),
      checkHealth: jasmine
        .createSpy('checkHealth')
        .and.returnValue(of({ rest: 'UP', rpc: 'DOWN', agent: 'UP' })),
    };

    await TestBed.configureTestingModule({
      imports: [DashboardComponent],
      providers: [
        { provide: SystemStateService, useValue: mockSystemStateService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(DashboardComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should call checkHealth on init', () => {
    expect(mockSystemStateService.checkHealth).toHaveBeenCalled();
  });

  it('should call checkHealth on refresh click', () => {
    mockSystemStateService.checkHealth.calls.reset();
    const btn = fixture.debugElement.query(By.css('app-button')).nativeElement;
    btn.click();
    expect(mockSystemStateService.checkHealth).toHaveBeenCalled();
  });

  it('should display correct health badges', () => {
    const badges = fixture.debugElement.queryAll(By.css('app-badge'));
    expect(badges.length).toBe(3);

    // REST (UP)
    expect(badges[0].nativeElement.textContent.trim()).toBe('UP');
    expect(badges[0].componentInstance.variant()).toBe('success');

    // RPC (DOWN)
    expect(badges[1].nativeElement.textContent.trim()).toBe('DOWN');
    expect(badges[1].componentInstance.variant()).toBe('danger');

    // Agent (UP)
    expect(badges[2].nativeElement.textContent.trim()).toBe('UP');
    expect(badges[2].componentInstance.variant()).toBe('success');
  });
});
