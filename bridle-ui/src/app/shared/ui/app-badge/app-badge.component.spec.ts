import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { AppBadgeComponent } from './app-badge.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-badge [variant]="variant">Test Badge</app-badge>`,
  imports: [AppBadgeComponent]
})
class TestHostComponent {
  variant: 'default' | 'success' | 'danger' | 'accent' = 'default';
}

describe('AppBadgeComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppBadgeComponent]
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
    expect(span.classList.contains('badge-default')).toBeTrue();
    
    component.variant = 'success';
    fixture.detectChanges();
    expect(span.classList.contains('badge-success')).toBeTrue();

    component.variant = 'danger';
    fixture.detectChanges();
    expect(span.classList.contains('badge-danger')).toBeTrue();

    component.variant = 'accent';
    fixture.detectChanges();
    expect(span.classList.contains('badge-accent')).toBeTrue();
  });
});
