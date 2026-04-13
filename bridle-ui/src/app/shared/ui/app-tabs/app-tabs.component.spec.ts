import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { AppTabsComponent, TabItem } from './app-tabs.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-tabs
    [tabs]="tabs"
    [activeTabId]="activeTabId"
    (tabChange)="onTabChange($event)"
  ></app-tabs>`,
  imports: [AppTabsComponent],
})
class TestHostComponent {
  tabs: TabItem[] = [
    { id: '1', label: 'Tab 1' },
    { id: '2', label: 'Tab 2', badge: 5 },
  ];
  activeTabId = '1';
  lastSelected = '';

  onTabChange(id: string) {
    this.lastSelected = id;
    this.activeTabId = id;
  }
}

describe('AppTabsComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppTabsComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render tabs and badges', () => {
    const buttons = fixture.debugElement.queryAll(By.css('button'));
    expect(buttons.length).toBe(2);
    expect(buttons[0].nativeElement.textContent).toContain('Tab 1');
    expect(buttons[1].nativeElement.textContent).toContain('Tab 2');
    expect(buttons[1].nativeElement.textContent).toContain('5');
  });

  it('should apply selected class to active tab', () => {
    const buttons = fixture.debugElement.queryAll(By.css('button'));
    expect(buttons[0].nativeElement.classList.contains('selected')).toBeTrue();
    expect(buttons[1].nativeElement.classList.contains('selected')).toBeFalse();
  });

  it('should emit tabChange event when clicking a tab', () => {
    const buttons = fixture.debugElement.queryAll(By.css('button'));
    buttons[1].nativeElement.click();
    fixture.detectChanges();
    expect(component.lastSelected).toBe('2');
    expect(component.activeTabId).toBe('2');

    // Check if the DOM updated
    const updatedButtons = fixture.debugElement.queryAll(By.css('button'));
    expect(
      updatedButtons[0].nativeElement.classList.contains('selected'),
    ).toBeFalse();
    expect(
      updatedButtons[1].nativeElement.classList.contains('selected'),
    ).toBeTrue();
  });
});
