import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component, signal } from '@angular/core';
import { AppBadgeComponent } from './app-badge.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-badge [variant]="variant()">Test Badge</app-badge>`,
  imports: [AppBadgeComponent],
})
class TestHostComponent {
  variant = signal<'default' | 'success' | 'danger' | 'accent'>('default');
}

describe('AppBadgeComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppBadgeComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render content', () => {
    const span = fixture.debugElement.query(By.css('span')).nativeElement;
    expect(span.textContent.trim()).toBe('Test Badge');
  });

  it('should set the correct variant class', () => {
    const span = fixture.debugElement.query(By.css('span')).nativeElement;
    expect(span.classList.contains('Label--default')).toBeTrue();

    component.variant.set('success');
    fixture.detectChanges();
    expect(span.classList.contains('Label--success')).toBeTrue();

    component.variant.set('danger');
    fixture.detectChanges();
    expect(span.classList.contains('Label--danger')).toBeTrue();

    component.variant.set('accent');
    fixture.detectChanges();
    expect(span.classList.contains('Label--accent')).toBeTrue();
  });
});
