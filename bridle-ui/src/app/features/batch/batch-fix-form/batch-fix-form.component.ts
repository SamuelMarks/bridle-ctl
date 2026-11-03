import { Component, output, inject, signal } from '@angular/core';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';
import { JsonObject } from '../../../core/models/models';

/**
 * Form component for creating a new batch fix job.
 */
@Component({
  selector: 'app-batch-fix-form',
  imports: [ReactiveFormsModule, AppButtonComponent, AppInputComponent],
  template: `
    <div class="Box">
      <div class="Box-header">
        <h2 class="Box-title">Create Batch Fix Job</h2>
      </div>
      <div class="Box-body">
        <form [formGroup]="form" (ngSubmit)="onSubmit()" class="batch-form">
          <app-input
            label="Organization ID"
            placeholder="e.g. org-123"
            formControlName="orgId"
          ></app-input>

          <app-input
            label="Issue Title"
            placeholder="e.g. Upgrade deprecated APIs"
            formControlName="title"
          ></app-input>

          <app-input
            type="textarea"
            label="Issue Description"
            placeholder="Describe the changes being made..."
            formControlName="description"
          ></app-input>

          <app-input
            label="Target Regex Pattern"
            placeholder="e.g. src/.*\\.rs"
            formControlName="pattern"
          ></app-input>

          <app-input
            label="Tools (Comma separated)"
            placeholder="e.g. clippy, rustfmt"
            formControlName="tools"
          ></app-input>

          <app-input
            type="textarea"
            label="Tool Arguments (JSON)"
            placeholder="{}"
            formControlName="args"
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
              id="fixSafetyMode"
              formControlName="safety_mode"
            />
            <label for="fixSafetyMode"
              >Safety Mode (Do not automatically fork and submit PRs)</label
            >
          </div>

          <div class="form-actions">
            <app-button
              type="submit"
              variant="primary"
              [disabled]="form.invalid || isSubmitting()"
            >
              {{ isSubmitting() ? 'Creating Job...' : 'Create Batch Fix' }}
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

    .batch-form {
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
export class BatchFixFormComponent {
  /** Injected FormBuilder */
  private fb = inject(FormBuilder);

  /** Emitted when form is submitted */
  createJob = output<{
    orgId: string;
    title: string;
    description: string;
    pattern: string;
    tools: string[];
    args: JsonObject;
    safety_mode: boolean;
    max_repos?: number;
    max_prs_per_hour?: number;
  }>();

  /** Reactive form group */
  form = this.fb.group({
    orgId: ['', Validators.required],
    title: ['', Validators.required],
    description: ['', Validators.required],
    pattern: ['', Validators.required],
    tools: ['', Validators.required],
    args: ['{}'],
    safety_mode: [true],
    max_repos: [''],
    max_prs_per_hour: [''],
  });

  /** Loading state */
  isSubmitting = signal(false);

  /** Handles form submission */
  onSubmit(): void {
    if (this.form.valid) {
      const toolsStr = this.form.value.tools || '';
      const tools = toolsStr
        .split(',')
        .map((t) => t.trim())
        .filter((t) => t.length > 0);

      let args: JsonObject = {};
      try {
        args = JSON.parse(this.form.value.args || '{}') as JsonObject;
      } catch {
        // Ignore parse error and fallback to empty object
      }

      const maxReposVal = this.form.value.max_repos;
      const maxPrsVal = this.form.value.max_prs_per_hour;

      this.createJob.emit({
        orgId: this.form.value.orgId!,
        title: this.form.value.title!,
        description: this.form.value.description!,
        pattern: this.form.value.pattern!,
        tools,
        args,
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
      this.form.reset({ args: '{}' });
    }
  }
}
