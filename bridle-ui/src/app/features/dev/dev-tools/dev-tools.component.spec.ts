import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ReactiveFormsModule } from '@angular/forms';
import { of, throwError } from 'rxjs';
import { DevToolsComponent } from './dev-tools.component';
import { ApiService } from '../../../core/services/api.service';
import { NotificationService } from '../../../core/services/notification.service';

interface MockNotificationService {
  success: jasmine.Spy;
  error: jasmine.Spy;
  info: jasmine.Spy;
}

interface MockApiService {
  post: jasmine.Spy;
}

describe('DevToolsComponent', () => {
  let component: DevToolsComponent;
  let fixture: ComponentFixture<DevToolsComponent>;
  let mockApiService: MockApiService;
  let mockNotificationService: MockNotificationService;

  beforeEach(async () => {
    mockApiService = {
      post: jasmine.createSpy('post').and.returnValue(of({ result: 42 })),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      error: jasmine.createSpy('error'),
      info: jasmine.createSpy('info'),
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
    component.addForm.setValue({ a: 10, b: 32 });
    component.onAdd();

    expect(mockApiService.post).toHaveBeenCalledWith('/dev/add', {
      a: 10,
      b: 32,
    });
    expect(component.addResult()).toBe(42);
  });

  it('should handle math addition error', () => {
    mockApiService.post.and.returnValue(throwError(() => new Error('err')));
    component.addForm.setValue({ a: 10, b: 32 });
    component.onAdd();

    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Calculation failed',
    );
  });

  it('should not add if form is invalid', () => {
    component.addForm.controls.a.setValue(null);
    component.onAdd();
    expect(mockApiService.post).not.toHaveBeenCalled();
  });

  it('should execute db command', () => {
    mockApiService.post.and.returnValue(of({ success: true }));

    component.dbForm.setValue({
      action: 'get_user',
      id: '123',
      payload: '{"key":"value"}',
    });

    component.onDbExec();

    expect(mockApiService.post).toHaveBeenCalledWith('/dev/db', {
      action: 'get_user',
      id: '123',
      payload: { key: 'value' },
    });
    expect(component.dbResult()).toEqual({ success: true });
    expect(mockNotificationService.success).toHaveBeenCalledWith(
      'Command executed',
    );
  });

  it('should handle invalid JSON payload', () => {
    component.dbForm.setValue({
      action: 'get_user',
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
      action: 'get_user',
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
