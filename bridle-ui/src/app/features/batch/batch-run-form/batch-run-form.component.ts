import { Component, output, inject, signal } from '@angular/core';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';

/**
 * Form component for running a batch pipeline from YAML configuration.
 */
@Component({
  selector: 'app-batch-run-form',
  imports: [ReactiveFormsModule, AppButtonComponent, AppInputComponent],
  template: `
    <div class="Box">
      <div class="Box-header">
        <h2 class="Box-title">Run Pipeline (YAML)</h2>
      </div>
      <div class="Box-body">
        <form [formGroup]="form" (ngSubmit)="onSubmit()" class="pipeline-form">
          <app-input
            type="textarea"
            label="Pipeline Configuration"
            placeholder="name: update-deps&#10;steps:&#10;  - run: cargo update"
            formControlName="configYaml"
            [rows]="10"
            [monospace]="true"
          ></app-input>

          <app-input
            type="number"
            label="Max Repositories (Optional)"
            placeholder="e.g. 50"
            formControlName="max_repos"
          ></app-input>

          <app-input
            type="number"
            label="Max PRs Per Hour (Optional)"
            placeholder="e.g. 10"
            formControlName="max_prs_per_hour"
          ></app-input>

          <div class="form-check">
            <input
              type="checkbox"
              id="runSafetyMode"
              formControlName="safety_mode"
            />
            <label for="runSafetyMode"
              >Safety Mode (Do not automatically fork and submit PRs)</label
            >
          </div>

          <div class="form-actions">
            <app-button
              type="submit"
              variant="primary"
              [disabled]="form.invalid || isSubmitting()"
            >
              {{ isSubmitting() ? 'Starting Pipeline...' : 'Run Pipeline' }}
            </app-button>
          </div>
        </form>
      </div>
    </div>
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }

    .Box-header {
      padding: 16px;
      background-color: var(--color-canvas-subtle);
      border-bottom: 1px solid var(--color-border-default);
      border-top-left-radius: var(--border-radius-2);
      border-top-right-radius: var(--border-radius-2);
    }

    .Box-title {
      font-size: 14px;
      font-weight: 600;
      margin: 0;
    }

    .Box-body {
      padding: 16px;
    }

    .pipeline-form {
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    .form-actions {
      display: flex;
      justify-content: flex-end;
      margin-top: 8px;
    }

    .form-check {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 14px;
    }
  `,
})
export class BatchRunFormComponent {
  /** Injected FormBuilder */
  private fb = inject(FormBuilder);

  /** Emitted when pipeline run is requested */
  runPipeline = output<{
    configYaml: string;
    safety_mode: boolean;
    max_repos?: number;
    max_prs_per_hour?: number;
  }>();

  /** Reactive form group */
  form = this.fb.group({
    configYaml: ['', Validators.required],
    safety_mode: [true],
    max_repos: [''],
    max_prs_per_hour: [''],
  });

  /** Loading state */
  isSubmitting = signal(false);

  /** Handles form submission */
  onSubmit(): void {
    if (this.form.valid) {
      const maxReposVal = this.form.value.max_repos;
      const maxPrsVal = this.form.value.max_prs_per_hour;

      this.runPipeline.emit({
        configYaml: this.form.value.configYaml!,
        safety_mode: !!this.form.value.safety_mode,
        max_repos: maxReposVal ? parseInt(maxReposVal, 10) : undefined,
        max_prs_per_hour: maxPrsVal ? parseInt(maxPrsVal, 10) : undefined,
      });
    }
  }

  /** Sets the submitting state */
  setSubmitting(isSubmitting: boolean): void {
    this.isSubmitting.set(isSubmitting);
    if (!isSubmitting && this.form.valid) {
      this.form.reset();
    }
  }
}
