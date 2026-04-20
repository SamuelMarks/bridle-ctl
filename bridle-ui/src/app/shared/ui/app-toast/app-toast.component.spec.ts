import { ComponentFixture, TestBed } from '@angular/core/testing';
import { AppToastComponent } from './app-toast.component';
import { NotificationService } from '../../../core/services/notification.service';

describe('AppToastComponent', () => {
  let component: AppToastComponent;
  let fixture: ComponentFixture<AppToastComponent>;
  let notificationService: NotificationService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AppToastComponent],
      providers: [NotificationService],
    }).compileComponents();

    fixture = TestBed.createComponent(AppToastComponent);
    component = fixture.componentInstance;
    notificationService = TestBed.inject(NotificationService);
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should call dismiss on the service when dismiss is called', () => {
    spyOn(notificationService, 'remove');
    component.dismiss('test-id');
    expect(notificationService.remove).toHaveBeenCalledWith('test-id');
  });

  it('should render notifications', () => {
    notificationService.success('Success message');
    notificationService.error('Error message');
    notificationService.info('Info message');
    fixture.detectChanges();

    const compiled = fixture.nativeElement as HTMLElement;
    const toasts = compiled.querySelectorAll('.Toast');
    expect(toasts.length).toBe(3);

    expect(toasts[0].classList.contains('Toast--success')).toBeTruthy();
    expect(toasts[0].textContent).toContain('Success message');

    expect(toasts[1].classList.contains('Toast--error')).toBeTruthy();
    expect(toasts[1].textContent).toContain('Error message');

    expect(toasts[2].classList.contains('Toast--info')).toBeTruthy();
    expect(toasts[2].textContent).toContain('Info message');
  });

  it('should dismiss notification on click', () => {
    notificationService.success('Success message');
    fixture.detectChanges();

    const compiled = fixture.nativeElement as HTMLElement;
    const button = compiled.querySelector(
      '.Toast-dismissButton',
    ) as HTMLButtonElement;

    spyOn(component, 'dismiss');
    button.click();

    expect(component.dismiss).toHaveBeenCalled();
  });
});
