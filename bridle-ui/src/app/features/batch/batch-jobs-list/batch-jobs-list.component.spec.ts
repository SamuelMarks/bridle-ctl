import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component, signal } from '@angular/core';
import { BatchJobsListComponent } from './batch-jobs-list.component';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-batch-jobs-list
    [jobs]="jobs()"
    (selectJob)="onSelect($event)"
  ></app-batch-jobs-list>`,
  imports: [BatchJobsListComponent],
})
class TestHostComponent {
  jobs = signal<BatchJob[]>([
    {
      id: '1234567890',
      target: 'org1/repo1',
      status: BatchJobStatus.COMPLETED,
      createdAt: '2023-01-01T00:00:00Z',
    },
    {
      id: '0987654321',
      target: 'org1',
      status: BatchJobStatus.FAILED,
      createdAt: '2023-01-02T00:00:00Z',
    },
  ]);
  selectedId = signal('');

  onSelect(id: string) {
    this.selectedId.set(id);
  }
}

describe('BatchJobsListComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, BatchJobsListComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render list with jobs and empty state', async () => {
    await fixture.whenStable();
    fixture.detectChanges();

    const listComponent = fixture.debugElement.query(
      By.directive(BatchJobsListComponent),
    ).componentInstance as BatchJobsListComponent;
    expect(listComponent.jobs().length).toBe(2);

    component.jobs.set([]);
    fixture.detectChanges();
    const emptyState = fixture.debugElement.query(By.css('.empty-state'));
    expect(emptyState).toBeTruthy();
  });

  it('should emit selectJob event', () => {
    const listComponent = fixture.debugElement.query(
      By.directive(BatchJobsListComponent),
    ).componentInstance as BatchJobsListComponent;
    listComponent.selectJob.emit(component.jobs()[0].id);

    expect(component.selectedId()).toBe('1234567890');
  });

  it('should return correct badge variants for status', () => {
    const listComponent = fixture.debugElement.query(
      By.directive(BatchJobsListComponent),
    ).componentInstance as BatchJobsListComponent;

    expect(listComponent.getBadgeVariant(BatchJobStatus.COMPLETED)).toBe(
      'success',
    );
    expect(listComponent.getBadgeVariant(BatchJobStatus.FAILED)).toBe('danger');
    expect(listComponent.getBadgeVariant(BatchJobStatus.RUNNING)).toBe(
      'accent',
    );
    expect(listComponent.getBadgeVariant(BatchJobStatus.PENDING)).toBe(
      'default',
    );
    expect(listComponent.getBadgeVariant(BatchJobStatus.INTERRUPTED)).toBe(
      'default',
    );
  });
});
