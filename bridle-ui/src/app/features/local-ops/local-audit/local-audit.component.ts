import { Component, output, inject, signal } from '@angular/core';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';
import { OpResult } from '../../../core/services/local-op.service';
import { JsonObject } from '../../../core/models/models';

/**
 * Component for running a local codebase audit.
 */
@Component({
  selector: 'app-local-audit',
  imports: [ReactiveFormsModule, AppButtonComponent, AppInputComponent],
  template: `
    <div class="audit-container">
      <div class="Box mb-4">
        <div class="Box-header">
          <h2 class="Box-title">Audit Configuration</h2>
        </div>
        <div class="Box-body">
          <form [formGroup]="form" (ngSubmit)="onSubmit()" class="op-form">
            <app-input
              label="Target Regex Pattern"
              placeholder="e.g. src/.*\\.ts"
              formControlName="pattern"
            ></app-input>

            <app-input
              label="Tools (Comma separated)"
              placeholder="e.g. eslint, prettier"
              formControlName="tools"
            ></app-input>

            <app-input
              type="textarea"
              label="Tool Arguments (JSON)"
              placeholder='{"eslint": ["--fix"]}'
              formControlName="args"
            ></app-input>

            <div class="form-actions">
              <app-button
                type="submit"
                variant="primary"
                [disabled]="form.invalid || isOperating()"
              >
                {{ isOperating() ? 'Running Audit...' : 'Run Audit' }}
              </app-button>
            </div>
          </form>
        </div>
      </div>

      @if (result()) {
        <div class="Box result-panel">
          <div class="Box-header">
            <h2 class="Box-title">Audit Output</h2>
          </div>
          <div class="Box-body p-0">
            <pre class="terminal-output">{{ result()!.output }}</pre>
          </div>
        </div>
      }
    </div>
  `,
  styles: `
    .mb-4 {
      margin-bottom: 24px;
    }
    .p-0 {
      padding: 0 !important;
    }

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

    .op-form {
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    .form-actions {
      display: flex;
      justify-content: flex-end;
      margin-top: 8px;
    }

    .terminal-output {
      margin: 0;
      padding: 16px;
      background-color: #24292f;
      color: #ffffff;
      font-family: var(--font-mono);
      font-size: 12px;
      line-height: 1.5;
      overflow-x: auto;
      border-bottom-left-radius: var(--border-radius-2);
      border-bottom-right-radius: var(--border-radius-2);
    }
  `,
})
export class LocalAuditComponent {
  /** Injected FormBuilder */
  private fb = inject(FormBuilder);

  /** Emitted when audit is requested */
  audit = output<{
    pattern: string;
    tools: string[];
    args: JsonObject;
  }>();

  /** Reactive form group */
  form = this.fb.group({
    pattern: ['', Validators.required],
    tools: [''],
    args: ['{}'],
  });

  /** Loading state */
  isOperating = signal(false);

  /** Result to display */
  result = signal<OpResult | null>(null);

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

      this.audit.emit({
        pattern: this.form.value.pattern!,
        tools,
        args,
      });
    }
  }

  /**
   * Sets the operating state.
   * @param isOperating Whether an operation is in progress
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
