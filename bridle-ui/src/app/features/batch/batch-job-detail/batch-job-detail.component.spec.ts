import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component, signal } from '@angular/core';
import { BatchJobDetailComponent } from './batch-job-detail.component';
import { BatchJob, BatchJobStatus } from '../../../core/models/models';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-batch-job-detail
    [job]="job()"
    (close)="onClose()"
    (resume)="onResume($event)"
  ></app-batch-job-detail>`,
  imports: [BatchJobDetailComponent],
})
class TestHostComponent {
  job = signal<BatchJob | null>({
    id: '1234567890',
    target: 'org1/repo1',
    status: BatchJobStatus.FAILED,
    createdAt: '2023-01-01T00:00:00Z',
  });
  closed = signal(false);
  resumedId = signal('');

  onClose() {
    this.closed.set(true);
  }

  onResume(id: string) {
    this.resumedId.set(id);
  }
}

describe('BatchJobDetailComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, BatchJobDetailComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render job details', () => {
    const title = fixture.debugElement.query(
      By.css('.Box-title'),
    ).nativeElement;
    expect(title.textContent).toContain('1234567890');

    const content = fixture.debugElement.query(
      By.css('.Box-body'),
    ).nativeElement;
    expect(content.textContent).toContain('org1/repo1');
    expect(content.textContent).toContain('FAILED');
  });

  it('should emit close event', () => {
    const closeBtn = fixture.debugElement.query(
      By.css('app-button'),
    ).nativeElement;
    closeBtn.click();
    expect(component.closed()).toBeTrue();
  });

  it('should render resume button if status is INTERRUPTED', () => {
    const currentJob = component.job();
    if (currentJob) {
      component.job.set({ ...currentJob, status: BatchJobStatus.INTERRUPTED });
    }
    fixture.detectChanges();

    const buttons = fixture.debugElement.queryAll(By.css('app-button'));
    expect(buttons.length).toBe(2);

    expect(buttons[1].nativeElement.textContent).toContain(
      'Resume Interrupted Job',
    );

    buttons[1].nativeElement.click();
    expect(component.resumedId()).toBe('1234567890');
  });

  it('should not render box if job is null', () => {
    component.job.set(null);
    fixture.detectChanges();

    const box = fixture.debugElement.query(By.css('.Box'));
    expect(box).toBeNull();
  });

  it('should return correct badge variants for status', () => {
    const detailComponent = fixture.debugElement.query(
      By.directive(BatchJobDetailComponent),
    ).componentInstance as BatchJobDetailComponent;

    expect(detailComponent.getBadgeVariant(BatchJobStatus.COMPLETED)).toBe(
      'success',
    );
    expect(detailComponent.getBadgeVariant(BatchJobStatus.FAILED)).toBe(
      'danger',
    );
    expect(detailComponent.getBadgeVariant(BatchJobStatus.RUNNING)).toBe(
      'accent',
    );
    expect(detailComponent.getBadgeVariant(BatchJobStatus.PENDING)).toBe(
      'default',
    );
    expect(detailComponent.getBadgeVariant(BatchJobStatus.INTERRUPTED)).toBe(
      'default',
    );
  });
});
