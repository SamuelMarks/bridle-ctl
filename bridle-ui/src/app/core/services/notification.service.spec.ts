import { TestBed, fakeAsync, tick } from '@angular/core/testing';
import { NotificationService } from './notification.service';

describe('NotificationService', () => {
  let service: NotificationService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(NotificationService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should add a success notification', () => {
    service.success('Test Success');
    const notifications = service.notifications();
    expect(notifications.length).toBe(1);
    expect(notifications[0].type).toBe('success');
    expect(notifications[0].message).toBe('Test Success');
  });

  it('should add an error notification', () => {
    service.error('Test Error');
    const notifications = service.notifications();
    expect(notifications.length).toBe(1);
    expect(notifications[0].type).toBe('error');
    expect(notifications[0].message).toBe('Test Error');
  });

  it('should add an info notification', () => {
    service.info('Test Info');
    const notifications = service.notifications();
    expect(notifications.length).toBe(1);
    expect(notifications[0].type).toBe('info');
    expect(notifications[0].message).toBe('Test Info');
  });

  it('should remove a notification by id', () => {
    service.info('Test Info');
    const id = service.notifications()[0].id;
    service.remove(id);
    expect(service.notifications().length).toBe(0);
  });

  it('should auto-remove notification after 5 seconds', fakeAsync(() => {
    service.info('Test Info');
    expect(service.notifications().length).toBe(1);
    
    tick(4999);
    expect(service.notifications().length).toBe(1);
    
    tick(1);
    expect(service.notifications().length).toBe(0);
  }));
});
