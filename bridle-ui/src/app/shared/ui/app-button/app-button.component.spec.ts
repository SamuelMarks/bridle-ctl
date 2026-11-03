import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component, signal } from '@angular/core';
import { AppButtonComponent } from './app-button.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-button
    [variant]="variant()"
    [type]="type()"
    [disabled]="disabled()"
    >Click Me</app-button
  >`,
  imports: [AppButtonComponent],
})
class TestHostComponent {
  variant = signal<'primary' | 'secondary' | 'danger' | 'invisible'>(
    'secondary',
  );
  type = signal<'button' | 'submit' | 'reset'>('button');
  disabled = signal(false);
}

describe('AppButtonComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppButtonComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render content', () => {
    const button = fixture.debugElement.query(By.css('button')).nativeElement;
    expect(button.textContent.trim()).toBe('Click Me');
  });

  it('should set the correct variant class', () => {
    const button = fixture.debugElement.query(By.css('button')).nativeElement;
    expect(button.classList.contains('btn-secondary')).toBeTrue();

    component.variant.set('primary');
    fixture.detectChanges();
    expect(button.classList.contains('btn-primary')).toBeTrue();

    component.variant.set('danger');
    fixture.detectChanges();
    expect(button.classList.contains('btn-danger')).toBeTrue();

    component.variant.set('invisible');
    fixture.detectChanges();
    expect(button.classList.contains('btn-invisible')).toBeTrue();
  });

  it('should set disabled state and attribute', () => {
    const button = fixture.debugElement.query(By.css('button')).nativeElement;
    expect(button.disabled).toBeFalse();
    expect(button.getAttribute('aria-disabled')).toBe('false');

    component.disabled.set(true);
    fixture.detectChanges();
    expect(button.disabled).toBeTrue();
    expect(button.getAttribute('aria-disabled')).toBe('true');
  });

  it('should set the button type', () => {
    const button = fixture.debugElement.query(By.css('button')).nativeElement;
    expect(button.type).toBe('button');

    component.type.set('submit');
    fixture.detectChanges();
    expect(button.type).toBe('submit');
  });
});
