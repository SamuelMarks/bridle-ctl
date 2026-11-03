import { signal, Component, output, inject } from '@angular/core';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';

/**
 * Form component for ingesting a new organization.
 */
@Component({
  selector: 'app-ingest-org-form',
  imports: [ReactiveFormsModule, AppButtonComponent, AppInputComponent],
  template: `
    <div class="Box">
      <div class="Box-header">
        <h2 class="Box-title">Ingest Organization</h2>
      </div>
      <div class="Box-body">
        <form [formGroup]="form" (ngSubmit)="onSubmit()" class="ingest-form">
          <app-input
            label="Organization Name"
            placeholder="e.g. google"
            formControlName="name"
          ></app-input>

          <app-input
            type="select"
            label="Provider"
            [options]="providerOptions"
            formControlName="provider"
          ></app-input>

          <app-input
            label="Database URL"
            placeholder="sqlite://bridle.db"
            formControlName="dbUrl"
          ></app-input>

          <div class="form-actions">
            <app-button
              type="submit"
              variant="primary"
              [disabled]="form.invalid || isSubmitting()"
            >
              {{ isSubmitting() ? 'Ingesting...' : 'Ingest Org' }}
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

    .ingest-form {
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    .form-actions {
      display: flex;
      justify-content: flex-end;
      margin-top: 8px;
    }
  `,
})
export class IngestOrgFormComponent {
  /** Injected FormBuilder */
  private fb = inject(FormBuilder);

  /** Emitted when the form is submitted with valid data */
  ingest = output<{ name: string; provider: string; dbUrl: string }>();

  /** Available provider options */
  providerOptions = [
    { label: 'GitHub', value: 'github' },
    { label: 'GitLab', value: 'gitlab' },
    { label: 'Bitbucket', value: 'bitbucket' },
  ];

  /** Reactive form group */
  form = this.fb.group({
    name: ['', Validators.required],
    provider: ['github', Validators.required],
    dbUrl: ['', Validators.required],
  });

  /** Internal loading state */
  isSubmitting = signal(false);

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
    this.isSubmitting.set(isSubmitting);
  }
}
