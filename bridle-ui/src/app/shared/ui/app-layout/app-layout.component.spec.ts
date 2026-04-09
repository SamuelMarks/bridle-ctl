import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { AppLayoutComponent } from './app-layout.component';
import { By } from '@angular/platform-browser';
import { provideRouter } from '@angular/router';

@Component({
  template: `<app-layout><div class="content">Test Content</div></app-layout>`,
  imports: [AppLayoutComponent]
})
class TestHostComponent {}

describe('AppLayoutComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppLayoutComponent],
      providers: [provideRouter([])]
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render header with title', () => {
    const title = fixture.debugElement.query(By.css('.Header-title')).nativeElement;
    expect(title.textContent.trim()).toBe('Bridle');
  });

  it('should render sidebar navigation', () => {
    const navItems = fixture.debugElement.queryAll(By.css('.SideNav-item'));
    expect(navItems.length).toBeGreaterThan(0);
    expect(navItems[0].nativeElement.textContent.trim()).toBe('Dashboard');
  });

  it('should render content', () => {
    const content = fixture.debugElement.query(By.css('.content')).nativeElement;
    expect(content.textContent.trim()).toBe('Test Content');
  });
});
