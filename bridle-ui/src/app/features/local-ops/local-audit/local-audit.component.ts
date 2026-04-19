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
import { OpResult } from '../../../core/services/local-op.service';

/**
 * Component for running a local audit operation.
 */
@Component({
  selector: 'app-local-audit',
  imports: [
    CommonModule,
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="Box p-3 mb-4">
      <h2 class="h4 mb-3">Run Audit</h2>
      <form [formGroup]="form" (ngSubmit)="onSubmit()">
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
            placeholder="e.g. grep, eslint"
          ></app-input>
        </div>

        <div class="mb-3">
          <app-input
            formControlName="args"
            type="textarea"
            label="Tool Arguments (JSON)"
            placeholder='{"grep": {"flags": "-i"}}'
          ></app-input>
        </div>

        <div class="d-flex justify-content-end">
          <app-button
            type="submit"
            variant="primary"
            [disabled]="form.invalid || isOperating()"
          >
            {{ isOperating() ? 'Auditing...' : 'Audit' }}
          </app-button>
        </div>
      </form>
    </div>

    @if (result()) {
      <div class="Box result-panel">
        <div class="Box-header">
          <h2 class="Box-title">Audit Results</h2>
        </div>
        <div class="Box-body p-0">
          <pre class="cli-output p-3 m-0">{{ result()?.output }}</pre>
        </div>
      </div>
    }
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }
    .Box-header {
      padding: 16px;
      margin: -1px -1px 0;
      background-color: var(--color-canvas-subtle);
      border: 1px solid var(--color-border-default);
      border-top-left-radius: var(--border-radius-2);
      border-top-right-radius: var(--border-radius-2);
    }
    .Box-title {
      font-size: 14px;
      font-weight: 600;
      margin: 0;
    }
    .Box-body {
      border-bottom: 1px solid var(--color-border-default);
    }
    .p-3 {
      padding: 16px;
    }
    .p-0 {
      padding: 0;
    }
    .m-0 {
      margin: 0;
    }
    .mb-3 {
      margin-bottom: 16px;
    }
    .mb-4 {
      margin-bottom: 24px;
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

    .cli-output {
      font-family: var(--font-family-mono);
      font-size: 12px;
      line-height: 1.45;
      background-color: var(--color-canvas-subtle);
      color: var(--color-fg-default);
      overflow-x: auto;
      white-space: pre-wrap;
    }
  `,
})
export class LocalAuditComponent {
  /** FormBuilder instance */
  private fb = inject(FormBuilder);

  /** Emitted when the form is submitted */
  audit = output<{
    pattern: string;
    tools: string[];
    args: Record<string, string | number | boolean | object | null | undefined>;
  }>();

  /** Form model */
  form = this.fb.group({
    pattern: ['', Validators.required],
    tools: [''],
    args: ['{}'],
  });

  /** Whether the operation is currently running */
  isOperating = signal<boolean>(false);

  /** Result of the operation */
  result = signal<OpResult | null>(null);

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

      this.audit.emit({
        pattern: this.form.value.pattern!,
        tools: toolsArr,
        args: parsedArgs,
      });
    }
  }

  /**
   * Sets the operating state.
   * @param isOperating Whether the operation is running
   */
  setOperating(isOperating: boolean): void {
    this.isOperating.set(isOperating);
  }

  /**
   * Sets the operation result.
   * @param result The result object
   */
  setResult(result: OpResult | null): void {
    this.result.set(result);
  }
}
