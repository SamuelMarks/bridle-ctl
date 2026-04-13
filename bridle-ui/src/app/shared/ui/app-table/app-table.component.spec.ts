import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import {
  AppTableComponent,
  AppTableColumnDirective,
} from './app-table.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `
    <app-table [title]="title" [data]="data" [emptyMessage]="emptyMessage">
      <ng-template appTableColumn title="ID" key="id" let-value="value">
        #{{ value }}
      </ng-template>
      <ng-template appTableColumn title="Name" key="name" let-row>
        {{ row.name }}
      </ng-template>
    </app-table>
  `,
  imports: [AppTableComponent, AppTableColumnDirective],
})
class TestHostComponent {
  title = 'Test Table';
  data: unknown[] = [
    { id: 1, name: 'Item 1' },
    { id: 2, name: 'Item 2' },
  ];
  emptyMessage = 'No data';
}

describe('AppTableComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppTableComponent, AppTableColumnDirective],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render the title', () => {
    const titleEl = fixture.debugElement.query(
      By.css('.Box-title'),
    ).nativeElement;
    expect(titleEl.textContent.trim()).toBe('Test Table');
  });

  it('should render columns', () => {
    const ths = fixture.debugElement.queryAll(By.css('th'));
    expect(ths.length).toBe(2);
    expect(ths[0].nativeElement.textContent.trim()).toBe('ID');
    expect(ths[1].nativeElement.textContent.trim()).toBe('Name');
  });

  it('should render rows with data', () => {
    const trs = fixture.debugElement.queryAll(By.css('tbody tr'));
    expect(trs.length).toBe(2);

    const firstRowTds = trs[0].queryAll(By.css('td'));
    expect(firstRowTds[0].nativeElement.textContent.trim()).toBe('#1');
    expect(firstRowTds[1].nativeElement.textContent.trim()).toBe('Item 1');
  });

  it('should render empty message when data is empty', () => {
    component.data = [];
    fixture.detectChanges();

    const trs = fixture.debugElement.queryAll(By.css('tbody tr'));
    expect(trs.length).toBe(1); // The empty message row
    expect(trs[0].nativeElement.textContent.trim()).toBe('No data');
  });

  it('should fallback to item in trackByFn if id is missing', () => {
    const tableComponent = fixture.debugElement.query(
      By.directive(AppTableComponent),
    ).componentInstance;
    const trackBy = tableComponent.trackByFn();
    expect(trackBy({ id: 123 })).toBe(123);
    const objWithoutId = { name: 'test' };
    expect(trackBy(objWithoutId)).toBe(objWithoutId);
  });
});
