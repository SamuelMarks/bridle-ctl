import { ComponentFixture, TestBed } from '@angular/core/testing';
import { IngestOrgFormComponent } from './ingest-org-form.component';
import { ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';

describe('IngestOrgFormComponent', () => {
  let component: IngestOrgFormComponent;
  let fixture: ComponentFixture<IngestOrgFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [IngestOrgFormComponent, ReactiveFormsModule],
    }).compileComponents();

    fixture = TestBed.createComponent(IngestOrgFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have an invalid form initially', () => {
    expect(component.form.valid).toBeFalse();
  });

  it('should emit ingest event on valid form submit', () => {
    spyOn(component.ingest, 'emit');

    component.form.setValue({
      name: 'test-org',
      provider: 'github',
      dbUrl: 'postgres://localhost/test',
    });

    component.onSubmit();

    expect(component.ingest.emit).toHaveBeenCalledWith({
      name: 'test-org',
      provider: 'github',
      dbUrl: 'postgres://localhost/test',
    });

    // Should reset form after successful submit
    expect(component.form.value.name).toBeNull();
    expect(component.form.value.provider).toBe('github');
  });

  it('should not emit ingest event if form is invalid', () => {
    spyOn(component.ingest, 'emit');
    component.onSubmit();
    expect(component.ingest.emit).not.toHaveBeenCalled();
  });

  it('should toggle isSubmitting state', () => {
    expect(component.isSubmitting).toBeFalse();
    component.setSubmitting(true);
    expect(component.isSubmitting).toBeTrue();
  });
});
