import {
  Component,
  ChangeDetectionStrategy,
  output,
  inject,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';

/**
 * Form component for ingesting a new organization.
 */
@Component({
  selector: 'app-ingest-org-form',
  imports: [
    CommonModule,
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="Box p-3">
      <h2 class="h4 mb-3">Ingest Organization</h2>
      <form [formGroup]="form" (ngSubmit)="onSubmit()">
        <div class="mb-3">
          <app-input
            formControlName="name"
            label="Organization Name"
            placeholder="e.g. example-org"
          ></app-input>
          @if (form.get('name')?.invalid && form.get('name')?.touched) {
            <div class="text-danger mt-1 text-small">Name is required</div>
          }
        </div>

        <div class="mb-3">
          <app-input
            formControlName="provider"
            label="Provider"
            type="select"
            [options]="providerOptions"
          ></app-input>
        </div>

        <div class="mb-3">
          <app-input
            formControlName="dbUrl"
            label="Database URL"
            placeholder="postgres://user:pass@localhost:5432/db"
          ></app-input>
          @if (form.get('dbUrl')?.invalid && form.get('dbUrl')?.touched) {
            <div class="text-danger mt-1 text-small">
              Database URL is required
            </div>
          }
        </div>

        <div class="d-flex justify-content-end">
          <app-button
            type="submit"
            variant="primary"
            [disabled]="form.invalid || isSubmitting"
          >
            {{ isSubmitting ? 'Ingesting...' : 'Ingest Org' }}
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
    .h4 {
      font-size: 16px;
      font-weight: 600;
      margin-top: 0;
    }
    .text-danger {
      color: var(--color-danger-fg);
    }
    .text-small {
      font-size: 12px;
    }
  `,
})
export class IngestOrgFormComponent {
  /** FormBuilder instance */
  private fb = inject(FormBuilder);

  /** Emitted when the form is valid and submitted */
  ingest = output<{ name: string; provider: string; dbUrl: string }>();

  /** Form model */
  form = this.fb.group({
    name: ['', Validators.required],
    provider: ['github', Validators.required],
    dbUrl: ['', Validators.required],
  });

  /** Provider options for the select input */
  providerOptions = [
    { label: 'GitHub', value: 'github' },
    { label: 'GitLab', value: 'gitlab' },
    { label: 'Bitbucket', value: 'bitbucket' },
  ];

  /** Whether the form is currently submitting */
  isSubmitting = false;

  /** Handles form submission */
  onSubmit(): void {
    if (this.form.valid) {
      this.ingest.emit({
        name: this.form.value.name!,
        provider: this.form.value.provider!,
        dbUrl: this.form.value.dbUrl!,
      });
      this.form.reset({ provider: 'github' });
    }
  }

  /** Sets the submitting state */
  setSubmitting(isSubmitting: boolean): void {
    this.isSubmitting = isSubmitting;
  }
}
