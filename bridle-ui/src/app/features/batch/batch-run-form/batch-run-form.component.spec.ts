import { ComponentFixture, TestBed } from '@angular/core/testing';
import { BatchRunFormComponent } from './batch-run-form.component';
import { ReactiveFormsModule } from '@angular/forms';

describe('BatchRunFormComponent', () => {
  let component: BatchRunFormComponent;
  let fixture: ComponentFixture<BatchRunFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BatchRunFormComponent, ReactiveFormsModule],
    }).compileComponents();

    fixture = TestBed.createComponent(BatchRunFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have invalid form initially', () => {
    expect(component.form.valid).toBeFalse();
  });

  it('should emit runPipeline event with config string', () => {
    spyOn(component.runPipeline, 'emit');

    component.form.setValue({
      config: 'name: Pipeline',
      safety_mode: true,
      max_repos: null,
      max_prs_per_hour: null,
    });

    component.onSubmit();

    expect(component.runPipeline.emit).toHaveBeenCalledWith({
      config: 'name: Pipeline',
      safety_mode: true,
      max_repos: undefined,
      max_prs_per_hour: undefined,
    });
  });

  it('should toggle isSubmitting state and reset form on success', () => {
    expect(component.isSubmitting()).toBeFalse();

    component.form.setValue({
      config: 'yaml',
      safety_mode: true,
      max_repos: null,
      max_prs_per_hour: null,
    });

    component.setSubmitting(true);
    expect(component.isSubmitting()).toBeTrue();

    component.setSubmitting(false);
    expect(component.isSubmitting()).toBeFalse();
    expect(component.form.value.config).toBeNull();
  });

  it('should not emit if form is invalid', () => {
    spyOn(component.runPipeline, 'emit');
    component.onSubmit();
    expect(component.runPipeline.emit).not.toHaveBeenCalled();
  });

  it('should not reset form if setSubmitting(false) is called but form is invalid', () => {
    component.form.reset();
    component.setSubmitting(false);
    expect(component.form.valid).toBeFalse();
  });

  it('should handle null values in optional fields', () => {
    spyOn(component.runPipeline, 'emit');

    component.form.setValue({
      config: 'test-config',
      safety_mode: null,
      max_repos: 5,
      max_prs_per_hour: 10,
    });

    component.onSubmit();

    expect(component.runPipeline.emit).toHaveBeenCalledWith({
      config: 'test-config',
      safety_mode: true,
      max_repos: 5,
      max_prs_per_hour: 10,
    });
  });
});
