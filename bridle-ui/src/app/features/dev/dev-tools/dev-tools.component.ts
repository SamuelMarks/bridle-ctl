import { Component, inject, signal } from '@angular/core';
import { JsonPipe } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { ApiService } from '../../../core/services/api.service';
import { NotificationService } from '../../../core/services/notification.service';
import { JsonObject } from '../../../core/models/models';
import { AppButtonComponent } from '../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../shared/ui/app-input/app-input.component';
import {
  AppTabsComponent,
  TabItem,
} from '../../../shared/ui/app-tabs/app-tabs.component';

/**
 * Component providing developer utilities and raw database operations.
 */
@Component({
  selector: 'app-dev-tools',
  imports: [
    JsonPipe,
    ReactiveFormsModule,
    AppButtonComponent,
    AppInputComponent,
    AppTabsComponent,
  ],
  template: `
    <div class="dev-tools-container">
      <div class="page-header">
        <h1 class="h2">Developer Tools & Database</h1>
      </div>

      <app-tabs
        [tabs]="tabs"
        [activeTab]="activeTab()"
        (tabChange)="activeTab.set($event)"
      >
        @if (activeTab() === 'add') {
          <div class="Box">
            <div class="Box-header">
              <h2 class="Box-title">Add Numbers (RPC/REST Test)</h2>
            </div>
            <div class="Box-body">
              <form [formGroup]="addForm" (ngSubmit)="onAdd()" class="dev-form">
                <div class="form-row">
                  <app-input
                    type="number"
                    label="First Number (a)"
                    formControlName="a"
                  ></app-input>
                  <app-input
                    type="number"
                    label="Second Number (b)"
                    formControlName="b"
                  ></app-input>
                </div>
                <div class="form-actions">
                  <app-button
                    type="submit"
                    variant="primary"
                    [disabled]="addForm.invalid || isAdding()"
                  >
                    Calculate Sum
                  </app-button>
                </div>
              </form>

              @if (addResult() !== null) {
                <div class="result-box mt-3">
                  <strong>Result:</strong> {{ addResult() }}
                </div>
              }
            </div>
          </div>
        } @else {
          <div class="Box">
            <div class="Box-header">
              <h2 class="Box-title">Raw Database Command</h2>
            </div>
            <div class="Box-body">
              <form
                [formGroup]="dbForm"
                (ngSubmit)="onDbExec()"
                class="dev-form"
              >
                <div class="form-row">
                  <app-input
                    type="select"
                    label="Action"
                    [options]="actionOptions"
                    formControlName="action"
                    class="flex-1"
                  ></app-input>
                  <app-input
                    label="Entity ID (Optional)"
                    placeholder="e.g. 123"
                    formControlName="id"
                    class="flex-1"
                  ></app-input>
                </div>

                <app-input
                  type="textarea"
                  label="JSON Payload"
                  placeholder='{"name": "test"}'
                  formControlName="payload"
                ></app-input>

                <div class="form-actions">
                  <app-button
                    type="submit"
                    variant="danger"
                    [disabled]="dbForm.invalid || isDbExecuting()"
                  >
                    Execute DB Command
                  </app-button>
                </div>
              </form>

              @if (dbResult()) {
                <div class="mt-3">
                  <strong>Response:</strong>
                  <pre class="json-output">{{ dbResult() | json }}</pre>
                </div>
              }
            </div>
          </div>
        }
      </app-tabs>
    </div>
  `,
  styles: `
    .dev-tools-container {
      max-width: 800px;
      margin: 0 auto;
    }

    .page-header {
      margin-bottom: 24px;
      padding-bottom: 16px;
      border-bottom: 1px solid var(--color-border-default);
    }

    .h2 {
      font-size: 24px;
      font-weight: 600;
      margin: 0;
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

    .dev-form {
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    .form-row {
      display: flex;
      gap: 16px;
    }

    .flex-1 {
      flex: 1;
    }

    .form-actions {
      display: flex;
      justify-content: flex-end;
    }

    .mt-3 {
      margin-top: 16px;
    }

    .result-box {
      padding: 12px;
      background-color: var(--color-canvas-subtle);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-1);
    }

    .json-output {
      margin: 8px 0 0 0;
      padding: 12px;
      background-color: #24292f;
      color: #ffffff;
      font-family: var(--font-mono);
      font-size: 12px;
      border-radius: var(--border-radius-1);
      overflow-x: auto;
    }
  `,
})
export class DevToolsComponent {
  /** Injected FormBuilder */
  private fb = inject(FormBuilder);
  /** Injected ApiService */
  private api = inject(ApiService);
  /** Injected NotificationService */
  private notificationService = inject(NotificationService);

  /** Available tabs */
  tabs: TabItem[] = [
    { id: 'add', label: 'Add Utility' },
    { id: 'db', label: 'DB Explorer' },
  ];

  /** Active tab ID */
  activeTab = signal('add');

  /** Add utility form */
  addForm = this.fb.group({
    a: [0, Validators.required],
    b: [0, Validators.required],
  });

  /** Loading state for add */
  isAdding = signal(false);
  /** Result of add */
  addResult = signal<number | null>(null);

  /** DB explorer form */
  dbForm = this.fb.group({
    action: ['get_user', Validators.required],
    id: [''],
    payload: ['{}'],
  });

  /** DB action options */
  actionOptions = [
    { label: 'Get User', value: 'get_user' },
    { label: 'Create User', value: 'create_user' },
    { label: 'Get Org', value: 'get_org' },
    { label: 'Create Org', value: 'create_org' },
    { label: 'Get Repo', value: 'get_repo' },
    { label: 'Create Repo', value: 'create_repo' },
  ];

  /** Loading state for DB command */
  isDbExecuting = signal(false);
  /** Result of DB command */
  dbResult = signal<object | null>(null);

  /** Handles add submission */
  onAdd(): void {
    if (this.addForm.valid) {
      this.isAdding.set(true);
      const a = Number(this.addForm.value.a);
      const b = Number(this.addForm.value.b);

      this.api.post<{ result: number }>('/dev/add', { a, b }).subscribe({
        next: (res) => {
          this.addResult.set(res.result);
          this.isAdding.set(false);
        },
        error: () => {
          this.notificationService.error('Calculation failed');
          this.isAdding.set(false);
        },
      });
    }
  }

  /** Handles DB command execution */
  onDbExec(): void {
    if (this.dbForm.valid) {
      this.isDbExecuting.set(true);
      let parsedPayload: JsonObject = {};
      try {
        parsedPayload = JSON.parse(
          this.dbForm.value.payload || '{}',
        ) as JsonObject;
      } catch {
        this.notificationService.error('Invalid JSON payload');
        this.isDbExecuting.set(false);
        return;
      }

      this.api
        .post<object>('/dev/db', {
          action: this.dbForm.value.action,
          id: this.dbForm.value.id || undefined,
          payload: parsedPayload,
        })
        .subscribe({
          next: (res) => {
            this.dbResult.set(res);
            this.notificationService.success('Command executed');
            this.isDbExecuting.set(false);
          },
          error: (err: Error) => {
            this.dbResult.set({ error: err.message });
            this.notificationService.error('Command failed');
            this.isDbExecuting.set(false);
          },
        });
    }
  }
}
