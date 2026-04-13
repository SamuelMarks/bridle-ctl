import { ComponentFixture, TestBed } from '@angular/core/testing';
import { DevToolsComponent } from './dev-tools.component';
import { ApiService } from '../../../core/services/api.service';
import { NotificationService } from '../../../core/services/notification.service';
import { ReactiveFormsModule } from '@angular/forms';
import { of, throwError } from 'rxjs';
import { By } from '@angular/platform-browser';

describe('DevToolsComponent', () => {
  let component: DevToolsComponent;
  let fixture: ComponentFixture<DevToolsComponent>;
  let mockApiService: any;
  let mockNotificationService: any;

  beforeEach(async () => {
    mockApiService = {
      post: jasmine.createSpy('post').and.returnValue(of({ result: 42 })),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      error: jasmine.createSpy('error'),
    };

    await TestBed.configureTestingModule({
      imports: [DevToolsComponent, ReactiveFormsModule],
      providers: [
        { provide: ApiService, useValue: mockApiService },
        { provide: NotificationService, useValue: mockNotificationService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(DevToolsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should perform math addition', () => {
    component.addForm.setValue({ left: 10, right: 32 });
    component.onAdd();

    expect(mockApiService.post).toHaveBeenCalledWith('/dev/add', {
      left: 10,
      right: 32,
    });
    expect(component.addResult()).toBe(42);
  });

  it('should handle math addition error', () => {
    mockApiService.post.and.returnValue(throwError(() => new Error('err')));
    component.addForm.setValue({ left: 10, right: 32 });
    component.onAdd();

    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Math operation failed',
    );
  });

  it('should not add if form is invalid', () => {
    component.addForm.setValue({ left: null, right: null } as any);
    component.onAdd();
    expect(mockApiService.post).not.toHaveBeenCalled();
  });

  it('should execute db command', () => {
    mockApiService.post.and.returnValue(of({ success: true }));

    component.dbForm.setValue({
      action: 'test_action',
      id: '123',
      payload: '{"key":"value"}',
    });

    component.onDbExec();

    expect(mockApiService.post).toHaveBeenCalledWith('/dev/db', {
      action: 'test_action',
      id: '123',
      payload: { key: 'value' },
    });
    expect(component.dbResult()).toEqual({ success: true });
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Command executed successfully',
    );
  });

  it('should handle invalid JSON payload', () => {
    component.dbForm.setValue({
      action: 'test_action',
      id: '',
      payload: '{invalid',
    });

    component.onDbExec();

    expect(mockApiService.post).not.toHaveBeenCalled();
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Invalid JSON payload',
    );
  });

  it('should handle db command error', () => {
    mockApiService.post.and.returnValue(throwError(() => new Error('Db err')));

    component.dbForm.setValue({
      action: 'test_action',
      id: '',
      payload: '',
    });

    component.onDbExec();

    expect(component.dbResult()).toEqual({ error: 'Db err' });
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Command failed',
    );
  });

  it('should not execute db command if form is invalid', () => {
    component.dbForm.setValue({ action: '', id: '', payload: '' });
    component.onDbExec();
    expect(mockApiService.post).not.toHaveBeenCalled();
  });
});
