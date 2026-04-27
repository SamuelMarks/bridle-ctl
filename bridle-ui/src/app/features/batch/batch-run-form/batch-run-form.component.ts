import {
  Component,
  ChangeDetectionStrategy,
  output,
  inject,
  signal,
} from '@angular/core';

import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';

/**
 * Form component for initiating a batch run operation via YAML config.
 */
@Component({
  selector: 'app-batch-run-form',
  imports: [
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent
],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="Box p-3">
      <form [formGroup]="form" (ngSubmit)="onSubmit()">
        <div class="mb-3">
          <app-input
            formControlName="config"
            type="textarea"
            label="Pipeline Configuration (YAML)"
            placeholder="name: My Pipeline&#10;jobs:&#10;  - name: Fix lint&#10;    tools: [eslint]"
          ></app-input>
          @if (form.get('config')?.invalid && form.get('config')?.touched) {
            <div class="text-danger mt-1 text-small">
              Configuration is required
            </div>
          }
        </div>

        <div class="mb-3">
          <app-input
            formControlName="max_repos"
            type="number"
            label="Max Repositories"
            placeholder="No limit"
          ></app-input>
        </div>

        <div class="mb-3">
          <app-input
            formControlName="max_prs_per_hour"
            type="number"
            label="Max PRs per Hour"
            placeholder="No limit"
          ></app-input>
        </div>

        <div class="mb-3 form-checkbox">
          <label>
            <input type="checkbox" formControlName="safety_mode" />
            Safety Mode (Dry-run PR submission)
          </label>
        </div>

        <div class="d-flex justify-content-end">
          <app-button
            type="submit"
            variant="primary"
            [disabled]="form.invalid || isSubmitting()"
          >
            {{ isSubmitting() ? 'Running Pipeline...' : 'Run Pipeline' }}
          </app-button>
        </div>
      </form>
    </div>
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }
    .p-3 {
      padding: 16px;
    }
    .mb-3 {
      margin-bottom: 16px;
    }
    .mt-1 {
      margin-top: 4px;
    }
    .text-danger {
      color: var(--color-danger-fg);
    }
    .text-small {
      font-size: 12px;
    }

    /* Make textarea taller for config */
    ::ng-deep app-input[formcontrolname='config'] textarea {
      min-height: 200px;
      font-family: var(--font-family-mono);
      font-size: 12px;
    }
  `,
})
export class BatchRunFormComponent {
  /** FormBuilder instance */
  private fb = inject(FormBuilder);

  /** Emitted when the form is submitted */
  runPipeline = output<{
    config: string;
    safety_mode: boolean;
    max_repos?: number;
    max_prs_per_hour?: number;
  }>();

  /** Form model */
  form = this.fb.group({
    config: ['', Validators.required],
    safety_mode: [true],
    max_repos: [null as number | null],
    max_prs_per_hour: [null as number | null],
  });

  /** Whether the form is currently submitting */
  isSubmitting = signal<boolean>(false);

  /** Handles form submission */
  onSubmit(): void {
    if (this.form.valid) {
      this.runPipeline.emit({
        config: this.form.value.config!,
        safety_mode: this.form.value.safety_mode ?? true,
        max_repos: this.form.value.max_repos ?? undefined,
        max_prs_per_hour: this.form.value.max_prs_per_hour ?? undefined,
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
