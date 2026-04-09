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
  let mockLocalOpService: any;
  let mockNotificationService: any;

  beforeEach(async () => {
    mockLocalOpService = {
      audit: jasmine.createSpy('audit').and.returnValue(of({ output: 'audit res' })),
      fix: jasmine.createSpy('fix').and.returnValue(of({ output: 'fix res' })),
      clearResult: jasmine.createSpy('clearResult')
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      info: jasmine.createSpy('info'),
      error: jasmine.createSpy('error')
    };

    await TestBed.configureTestingModule({
      imports: [LocalOpsPageComponent, NoopAnimationsModule],
      providers: [
        { provide: LocalOpService, useValue: mockLocalOpService },
        { provide: NotificationService, useValue: mockNotificationService }
      ]
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
    component.auditComponent = undefined as any;
    component.fixComponent = undefined as any;
    component.onTabChange('fix');
    expect(mockLocalOpService.clearResult).toHaveBeenCalled();
  });

  it('should clear results on tab change when components are defined', () => {
    component.auditComponent = { setResult: jasmine.createSpy() } as any;
    component.fixComponent = { setResult: jasmine.createSpy() } as any;
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
    mockLocalOpService.audit.and.returnValue(throwError(() => new Error('err')));
    component.onAudit({ pattern: 'test', tools: [], args: {} });
    expect(mockNotificationService.error).toHaveBeenCalledWith('Audit operation failed');
  });

  it('should call fix service and handle success for dry run', () => {
    component.onTabChange('fix');
    fixture.detectChanges();
    
    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: true });
    expect(mockLocalOpService.fix).toHaveBeenCalledWith('test', [], {}, true);
    expect(mockNotificationService.info).toHaveBeenCalledWith('Dry run completed');
  });

  it('should call fix service and handle success for actual run', () => {
    component.onTabChange('fix');
    fixture.detectChanges();
    
    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: false });
    expect(mockLocalOpService.fix).toHaveBeenCalledWith('test', [], {}, false);
    expect(mockNotificationService.success).toHaveBeenCalledWith('Fix operation completed');
  });

  it('should handle fix service error', () => {
    component.onTabChange('fix');
    fixture.detectChanges();
    
    mockLocalOpService.fix.and.returnValue(throwError(() => new Error('err')));
    component.onFix({ pattern: 'test', tools: [], args: {}, dryRun: false });
    expect(mockNotificationService.error).toHaveBeenCalledWith('Fix operation failed');
  });
});
