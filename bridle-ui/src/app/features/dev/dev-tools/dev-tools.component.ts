import {
  Component,
  ChangeDetectionStrategy,
  inject,
  signal,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { ApiService } from '../../../core/services/api.service';
import { NotificationService } from '../../../core/services/notification.service';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';

/**
 * Component for developer tools and raw commands.
 */
@Component({
  selector: 'app-dev-tools',
  imports: [
    CommonModule,
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="container-lg">
      <div class="mb-4">
        <h1 class="mb-2">Developer Tools</h1>
        <p class="text-muted">
          Direct interface to raw daemon commands and utility functions.
        </p>
      </div>

      <div class="layout-grid">
        <!-- Math Add utility -->
        <div class="Box p-3">
          <h2 class="h4 mb-3">Math Utility (Test RPC)</h2>
          <form [formGroup]="addForm" (ngSubmit)="onAdd()">
            <div class="d-flex gap-3 align-items-end mb-3">
              <div class="flex-1">
                <app-input
                  formControlName="left"
                  label="Left Integer"
                  type="number"
                ></app-input>
              </div>
              <div class="mb-2 text-bold">+</div>
              <div class="flex-1">
                <app-input
                  formControlName="right"
                  label="Right Integer"
                  type="number"
                ></app-input>
              </div>
              <div>
                <app-button
                  type="submit"
                  variant="secondary"
                  [disabled]="addForm.invalid || isAdding()"
                >
                  {{ isAdding() ? 'Calculating...' : 'Add' }}
                </app-button>
              </div>
            </div>

            @if (addResult() !== null) {
              <div class="result-badge success-bg">
                Result: <span class="text-bold">{{ addResult() }}</span>
              </div>
            }
          </form>
        </div>

        <!-- Raw DB Execution -->
        <div class="Box p-3">
          <h2 class="h4 mb-3">Raw DB Command</h2>
          <form [formGroup]="dbForm" (ngSubmit)="onDbExec()">
            <div class="mb-3">
              <app-input
                formControlName="action"
                label="Action"
                placeholder="e.g. read_orgs, delete_repo"
              ></app-input>
              @if (
                dbForm.get('action')?.invalid && dbForm.get('action')?.touched
              ) {
                <div class="text-danger mt-1 text-small">
                  Action is required
                </div>
              }
            </div>

            <div class="mb-3">
              <app-input
                formControlName="id"
                label="Entity ID (Optional)"
                placeholder="e.g. 12345"
              ></app-input>
            </div>

            <div class="mb-3">
              <app-input
                formControlName="payload"
                type="textarea"
                label="JSON Payload (Optional)"
                placeholder="{}"
              ></app-input>
            </div>

            <div class="d-flex justify-content-end mb-3">
              <app-button
                type="submit"
                variant="danger"
                [disabled]="dbForm.invalid || isDbExecuting()"
              >
                {{ isDbExecuting() ? 'Executing...' : 'Execute Raw Command' }}
              </app-button>
            </div>
          </form>

          @if (dbResult()) {
            <div class="mt-3 border-top pt-3">
              <h3 class="h5 mb-2">Result</h3>
              <pre class="cli-output p-3">{{ dbResult() | json }}</pre>
            </div>
          }
        </div>
      </div>
    </div>
  `,
  styles: `
    .mb-2 {
      margin-bottom: 8px;
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
    .mt-3 {
      margin-top: 16px;
    }
    .pt-3 {
      padding-top: 16px;
    }
    .p-3 {
      padding: 16px;
    }

    .text-muted {
      color: var(--color-fg-muted);
    }
    .text-bold {
      font-weight: 600;
    }
    .text-danger {
      color: var(--color-danger-fg);
    }
    .text-small {
      font-size: 12px;
    }
    .h4 {
      font-size: 16px;
      font-weight: 600;
      margin-top: 0;
    }
    .h5 {
      font-size: 14px;
      font-weight: 600;
      margin-top: 0;
    }

    .flex-1 {
      flex: 1;
    }
    .gap-3 {
      gap: 16px;
    }
    .align-items-end {
      align-items: flex-end;
    }
    .border-top {
      border-top: 1px solid var(--color-border-default);
    }

    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }

    .layout-grid {
      display: grid;
      gap: 24px;
    }

    .result-badge {
      display: inline-block;
      padding: 8px 16px;
      border-radius: var(--border-radius-2);
      font-size: 14px;
    }

    .success-bg {
      background-color: rgba(45, 164, 78, 0.1);
      color: var(--color-success-fg);
      border: 1px solid rgba(45, 164, 78, 0.4);
    }

    .cli-output {
      font-family: var(--font-family-mono);
      font-size: 12px;
      line-height: 1.45;
      background-color: var(--color-canvas-subtle);
      color: var(--color-fg-default);
      overflow-x: auto;
      white-space: pre-wrap;
      margin: 0;
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }
  `,
})
export class DevToolsComponent {
  /** Form builder instance */
  private fb = inject(FormBuilder);
  /** API service instance */
  private api = inject(ApiService);
  /** Notification service instance */
  private notificationService = inject(NotificationService);

  /** Add form */
  addForm = this.fb.group({
    left: [0, Validators.required],
    right: [0, Validators.required],
  });

  /** DB form */
  dbForm = this.fb.group({
    action: ['', Validators.required],
    id: [''],
    payload: [''],
  });

  /** Adding state */
  isAdding = signal<boolean>(false);
  /** Add result */
  addResult = signal<number | null>(null);

  /** DB executing state */
  isDbExecuting = signal<boolean>(false);
  /** DB result */
  dbResult = signal<Record<
    string,
    string | number | boolean | object | null | undefined
  > | null>(null);

  /** Handles math add */
  onAdd(): void {
    if (this.addForm.valid) {
      this.isAdding.set(true);
      const payload = {
        left: Number(this.addForm.value.left),
        right: Number(this.addForm.value.right),
      };

      this.api.post<{ result: number }>('/dev/add', payload).subscribe({
        next: (res) => {
          this.addResult.set(res.result);
          this.isAdding.set(false);
        },
        error: () => {
          this.notificationService.error('Math operation failed');
          this.isAdding.set(false);
        },
      });
    }
  }

  /** Handles raw DB command */
  onDbExec(): void {
    if (this.dbForm.valid) {
      this.isDbExecuting.set(true);

      let payloadObj = null;
      if (this.dbForm.value.payload) {
        try {
          payloadObj = JSON.parse(this.dbForm.value.payload);
        } catch (e) {
          this.notificationService.error('Invalid JSON payload');
          this.isDbExecuting.set(false);
          return;
        }
      }

      const reqBody = {
        action: this.dbForm.value.action,
        id: this.dbForm.value.id || undefined,
        payload: payloadObj,
      };

      this.api
        .post<
          Record<string, string | number | boolean | object | null | undefined>
        >('/dev/db', reqBody)
        .subscribe({
          next: (res) => {
            this.dbResult.set(res);
            this.isDbExecuting.set(false);
            this.notificationService.success('Command executed successfully');
          },
          error: (err) => {
            this.dbResult.set({ error: err.message });
            this.notificationService.error('Command failed');
            this.isDbExecuting.set(false);
          },
        });
    }
  }
}
