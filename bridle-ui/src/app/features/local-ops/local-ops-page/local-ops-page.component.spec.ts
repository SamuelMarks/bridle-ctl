import { ComponentFixture, TestBed } from '@angular/core/testing';
import { By } from '@angular/platform-browser';
import { of, throwError } from 'rxjs';
import { LocalOpsPageComponent } from './local-ops-page.component';
import { LocalOpService } from '../../../core/services/local-op.service';
import { NotificationService } from '../../../core/services/notification.service';

interface MockNotificationService {
  success: jasmine.Spy;
  error: jasmine.Spy;
  info: jasmine.Spy;
}

interface MockLocalOpService {
  audit: jasmine.Spy;
  fix: jasmine.Spy;
  clearResult: jasmine.Spy;
}

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
    };

    await TestBed.configureTestingModule({
      imports: [LocalOpsPageComponent],
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

    expect(mockLocalOpService.clearResult).toHaveBeenCalled();

    const audit = fixture.debugElement.query(By.css('app-local-audit'));
    expect(audit).toBeNull();

    const fix = fixture.debugElement.query(By.css('app-local-fix'));
    expect(fix).toBeTruthy();
  });

  it('should call audit service and handle success', () => {
    component.onAudit({ pattern: 'test', tools: [], args: {} });
    expect(mockLocalOpService.audit).toHaveBeenCalledWith('test', [], {});
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Audit completed',
    );
  });

  it('should handle audit service error', () => {
    mockLocalOpService.audit.and.returnValue(
      throwError(() => new Error('err')),
    );
    component.onAudit({ pattern: 'test', tools: [], args: {} });
    expect(mockNotificationService.error).toHaveBeenCalledWith('Audit failed');
  });

  it('should call fix service and handle success', () => {
    component.onTabChange('fix');
    fixture.detectChanges();

    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: true });
    expect(mockLocalOpService.fix).toHaveBeenCalledWith('test', [], {}, true);
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
