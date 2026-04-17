import {
  Component,
  ChangeDetectionStrategy,
  output,
  inject,
  signal,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';

/**
 * Form component for initiating a batch fix operation.
 */
@Component({
  selector: 'app-batch-fix-form',
  imports: [
    CommonModule,
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="Box p-3">
      <form [formGroup]="form" (ngSubmit)="onSubmit()">
        <div class="mb-3">
          <app-input
            formControlName="target"
            label="Target Organization/Repository"
            placeholder="e.g. google/bridle"
          ></app-input>
          @if (form.get('target')?.invalid && form.get('target')?.touched) {
            <div class="text-danger mt-1 text-small">Target is required</div>
          }
        </div>

        <div class="mb-3">
          <app-input
            formControlName="title"
            label="Issue/PR Title"
            placeholder="e.g. Update deprecated dependency"
          ></app-input>
          @if (form.get('title')?.invalid && form.get('title')?.touched) {
            <div class="text-danger mt-1 text-small">Title is required</div>
          }
        </div>

        <div class="mb-3">
          <app-input
            formControlName="description"
            type="textarea"
            label="Issue/PR Description"
            placeholder="Detailed description of the change..."
          ></app-input>
        </div>

        <div class="mb-3">
          <app-input
            formControlName="pattern"
            label="Regex Pattern"
            placeholder="e.g. TODO.*"
          ></app-input>
          @if (form.get('pattern')?.invalid && form.get('pattern')?.touched) {
            <div class="text-danger mt-1 text-small">Pattern is required</div>
          }
        </div>

        <div class="mb-3">
          <app-input
            formControlName="tools"
            label="Tools (comma-separated)"
            placeholder="e.g. sed, replace"
          ></app-input>
        </div>

        <div class="mb-3">
          <app-input
            formControlName="args"
            type="textarea"
            label="Tool Arguments (JSON)"
            placeholder='{"replace": {"new_string": "FIXED"}}'
          ></app-input>
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
            {{ isSubmitting() ? 'Queueing Job...' : 'Queue Batch Fix' }}
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
  `,
})
export class BatchFixFormComponent {
  /** FormBuilder instance */
  private fb = inject(FormBuilder);

  /** Emitted when the form is submitted */
  createJob = output<{
    target: string;
    title: string;
    description: string;
    pattern: string;
    tools: string[];
    args: Record<string, string | number | boolean | object | null | undefined>;
    safety_mode: boolean;
    max_repos?: number;
    max_prs_per_hour?: number;
  }>();

  /** Form model */
  form = this.fb.group({
    target: ['', Validators.required],
    title: ['', Validators.required],
    description: [''],
    pattern: ['', Validators.required],
    tools: [''],
    args: ['{}'],
    safety_mode: [true],
    max_repos: [null as number | null],
    max_prs_per_hour: [null as number | null],
  });

  /** Whether the form is currently submitting */
  isSubmitting = signal<boolean>(false);

  /** Handles form submission */
  onSubmit(): void {
    if (this.form.valid) {
      let toolsArr: string[] = [];
      if (this.form.value.tools) {
        toolsArr = this.form.value.tools
          .split(',')
          .map((t) => t.trim())
          .filter((t) => t.length > 0);
      }

      let parsedArgs = {};
      try {
        if (this.form.value.args) {
          parsedArgs = JSON.parse(this.form.value.args);
        }
      } catch (e) {
        // Just send empty if parsing fails for now
      }

      this.createJob.emit({
        target: this.form.value.target!,
        title: this.form.value.title!,
        description: this.form.value.description || '',
        pattern: this.form.value.pattern!,
        tools: toolsArr,
        args: parsedArgs,
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
      this.form.reset({ args: '{}' });
    }
  }
}
