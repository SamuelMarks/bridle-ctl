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
import { LocalOpsPageComponent } from './local-ops-page.component';
import { LocalOpService } from '../../../core/services/local-op.service';
import { NotificationService } from '../../../core/services/notification.service';
import { of, throwError } from 'rxjs';
import { By } from '@angular/platform-browser';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

describe('LocalOpsPageComponent', () => {
  let component: LocalOpsPageComponent;
  let fixture: ComponentFixture<LocalOpsPageComponent>;
  let mockLocalOpService: MockLocalOpService;
  let mockNotificationService: MockNotificationService;

  beforeEach(async () => {
    mockLocalOpService = {
      audit: jasmine
        .createSpy('audit')
        .and.returnValue(of({ output: 'audit res' })),
      fix: jasmine.createSpy('fix').and.returnValue(of({ output: 'fix res' })),
      clearResult: jasmine.createSpy('clearResult'),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      info: jasmine.createSpy('info'),
      error: jasmine.createSpy('error'),
    } as object as MockNotificationService;

    await TestBed.configureTestingModule({
      imports: [LocalOpsPageComponent, NoopAnimationsModule],
      providers: [
        { provide: LocalOpService, useValue: mockLocalOpService },
        { provide: NotificationService, useValue: mockNotificationService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(LocalOpsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render tabs', () => {
    const tabs = fixture.debugElement.query(By.css('app-tabs'));
    expect(tabs).toBeTruthy();
  });

  it('should render audit component initially', () => {
    const audit = fixture.debugElement.query(By.css('app-local-audit'));
    expect(audit).toBeTruthy();

    const fix = fixture.debugElement.query(By.css('app-local-fix'));
    expect(fix).toBeNull();
  });

  it('should switch to fix component on tab change', () => {
    component.onTabChange('fix');
    fixture.detectChanges();

    const audit = fixture.debugElement.query(By.css('app-local-audit'));
    expect(audit).toBeNull();

    const fix = fixture.debugElement.query(By.css('app-local-fix'));
    expect(fix).toBeTruthy();
  });

  it('should clear results on tab change when components are undefined', () => {
    // Force undefined
    Object.assign(component, { auditComponent: undefined });
    Object.assign(component, { fixComponent: undefined });
    component.onTabChange('fix');
    expect(mockLocalOpService.clearResult).toHaveBeenCalled();
  });

  it('should clear results on tab change when components are defined', () => {
    component.auditComponent = {
      setResult: jasmine.createSpy(),
    } as object as import('../local-audit/local-audit.component').LocalAuditComponent;
    component.fixComponent = {
      setResult: jasmine.createSpy(),
    } as object as import('../local-fix/local-fix.component').LocalFixComponent;
    component.onTabChange('fix');
    expect(mockLocalOpService.clearResult).toHaveBeenCalled();
    expect(component.auditComponent.setResult).toHaveBeenCalledWith(null);
    expect(component.fixComponent.setResult).toHaveBeenCalledWith(null);
  });

  it('should call audit service and handle success', () => {
    component.onAudit({ pattern: 'test', tools: [], args: {} });
    expect(mockLocalOpService.audit).toHaveBeenCalledWith('test', [], {});
    expect(component.auditComponent).toBeDefined();
    // Assuming audit component handled setting result
  });

  it('should handle audit service error', () => {
    mockLocalOpService.audit.and.returnValue(
      throwError(() => new Error('err')),
    );
    component.onAudit({ pattern: 'test', tools: [], args: {} });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Audit operation failed',
    );
  });

  it('should call fix service and handle success for dry run', () => {
    component.onTabChange('fix');
    fixture.detectChanges();

    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: true });
    expect(mockLocalOpService.fix).toHaveBeenCalledWith('test', [], {}, true);
    expect(mockNotificationService.info).toHaveBeenCalledWith(
      'Dry run completed',
    );
  });

  it('should call fix service and handle success for actual run', () => {
    component.onTabChange('fix');
    fixture.detectChanges();

    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: false });
    expect(mockLocalOpService.fix).toHaveBeenCalledWith('test', [], {}, false);
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Fix operation completed',
    );
  });

  it('should handle fix service error', () => {
    component.onTabChange('fix');
    fixture.detectChanges();

    mockLocalOpService.fix.and.returnValue(throwError(() => new Error('err')));
    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: false });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Fix operation failed',
    );
  });
});
