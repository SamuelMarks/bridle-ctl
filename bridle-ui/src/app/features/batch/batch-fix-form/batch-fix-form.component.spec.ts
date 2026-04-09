import { ComponentFixture, TestBed } from '@angular/core/testing';
import { BatchFixFormComponent } from './batch-fix-form.component';
import { ReactiveFormsModule } from '@angular/forms';

describe('BatchFixFormComponent', () => {
  let component: BatchFixFormComponent;
  let fixture: ComponentFixture<BatchFixFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BatchFixFormComponent, ReactiveFormsModule]
    }).compileComponents();

    fixture = TestBed.createComponent(BatchFixFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have invalid form initially', () => {
    expect(component.form.valid).toBeFalse();
  });

  it('should emit createJob event with parsed args', () => {
    spyOn(component.createJob, 'emit');

    component.form.setValue({
      target: 'org1',
      title: 'Fix things',
      description: 'Desc',
      pattern: 'TODO',
      tools: 'sed, replace',
      args: '{"replace": {"new_string": "FIXED"}}',
      safety_mode: true,
      max_repos: null,
      max_prs_per_hour: null
    });

    component.onSubmit();

    expect(component.createJob.emit).toHaveBeenCalledWith({
      target: 'org1',
      title: 'Fix things',
      description: 'Desc',
      pattern: 'TODO',
      tools: ['sed', 'replace'],
      args: { replace: { new_string: 'FIXED' } },
      safety_mode: true,
      max_repos: undefined,
      max_prs_per_hour: undefined
    });
  });

  it('should handle null description', () => {
    spyOn(component.createJob, 'emit');

    component.form.setValue({
      target: 'org1',
      title: 'Fix things',
      description: null,
      pattern: 'TODO',
      tools: '',
      args: '{}',
      safety_mode: false,
      max_repos: 10,
      max_prs_per_hour: 2
    });

    component.onSubmit();

    expect(component.createJob.emit).toHaveBeenCalledWith({
      target: 'org1',
      title: 'Fix things',
      description: '',
      pattern: 'TODO',
      tools: [],
      args: {},
      safety_mode: false,
      max_repos: 10,
      max_prs_per_hour: 2
    });
  });
  it('should handle invalid JSON in args', () => {
    spyOn(component.createJob, 'emit');

    component.form.setValue({
      target: 'org1',
      title: 'Fix things',
      description: 'Desc',
      pattern: 'TODO',
      tools: '',
      args: '{invalid json}',
      safety_mode: true,
      max_repos: null,
      max_prs_per_hour: null
    });

    component.onSubmit();

    expect(component.createJob.emit).toHaveBeenCalledWith({
      target: 'org1',
      title: 'Fix things',
      description: 'Desc',
      pattern: 'TODO',
      tools: [],
      args: {},
      safety_mode: true,
      max_repos: undefined,
      max_prs_per_hour: undefined
    });
  });

  it('should toggle isSubmitting state and reset form on success', () => {
    expect(component.isSubmitting()).toBeFalse();
    
    component.form.setValue({
      target: 'org1',
      title: 'Fix things',
      description: '',
      pattern: 'TODO',
      tools: '',
      args: '{}',
      safety_mode: true,
      max_repos: null,
      max_prs_per_hour: null
    });

    component.setSubmitting(true);
    expect(component.isSubmitting()).toBeTrue();

    component.setSubmitting(false);
    expect(component.isSubmitting()).toBeFalse();
    expect(component.form.value.target).toBeNull();
    expect(component.form.value.args).toBe('{}');
  });

  it('should not emit if form is invalid', () => {
    spyOn(component.createJob, 'emit');
    component.onSubmit();
    expect(component.createJob.emit).not.toHaveBeenCalled();
  });

  it('should not reset form if setSubmitting(false) is called but form is invalid', () => {
    component.form.reset();
    component.setSubmitting(false);
    expect(component.form.valid).toBeFalse();
    expect(component.form.value.args).toBeNull();
  });

  it('should handle falsy args and safety_mode', () => {
    spyOn(component.createJob, 'emit');

    component.form.patchValue({
      target: 'org1',
      title: 'Fix things',
      pattern: 'TODO',
      args: null,
      safety_mode: null,
      tools: 'sed, ,'
    });

    component.onSubmit();

    expect(component.createJob.emit).toHaveBeenCalledWith(jasmine.objectContaining({
      args: {},
      safety_mode: true,
      tools: ['sed']
    }));
  });
});

