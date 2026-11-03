import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component, signal } from '@angular/core';
import { AppTabsComponent, TabItem } from './app-tabs.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-tabs
    [tabs]="tabs()"
    [activeTab]="activeTab()"
    (tabChange)="onTabChange($event)"
  ></app-tabs>`,
  imports: [AppTabsComponent],
})
class TestHostComponent {
  tabs = signal<TabItem[]>([
    { id: '1', label: 'Tab 1' },
    { id: '2', label: 'Tab 2' },
  ]);
  activeTab = signal('1');
  lastSelected = signal('');

  onTabChange(id: string) {
    this.lastSelected.set(id);
    this.activeTab.set(id);
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

  it('should render tabs', () => {
    const buttons = fixture.debugElement.queryAll(By.css('button'));
    expect(buttons.length).toBe(2);
    expect(buttons[0].nativeElement.textContent).toContain('Tab 1');
    expect(buttons[1].nativeElement.textContent).toContain('Tab 2');
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
    expect(component.lastSelected()).toBe('2');
    expect(component.activeTab()).toBe('2');

    const updatedButtons = fixture.debugElement.queryAll(By.css('button'));
    expect(
      updatedButtons[0].nativeElement.classList.contains('selected'),
    ).toBeFalse();
    expect(
      updatedButtons[1].nativeElement.classList.contains('selected'),
    ).toBeTrue();
  });
});
