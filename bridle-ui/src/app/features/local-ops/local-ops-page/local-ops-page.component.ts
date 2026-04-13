import {
  Component,
  ChangeDetectionStrategy,
  inject,
  ViewChild,
  signal,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import {
  LocalOpService,
  OpResult,
} from '../../../core/services/local-op.service';
import { NotificationService } from '../../../core/services/notification.service';
import {
  AppTabsComponent,
  TabItem,
} from '../../../shared/ui/app-tabs/app-tabs.component';
import { LocalAuditComponent } from '../local-audit/local-audit.component';
import { LocalFixComponent } from '../local-fix/local-fix.component';

/**
 * Main page component for local operations (Audit/Fix).
 */
@Component({
  selector: 'app-local-ops-page',
  imports: [
    CommonModule,
    AppTabsComponent,
    LocalAuditComponent,
    LocalFixComponent,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="container-lg">
      <div class="mb-4">
        <h2 class="mb-2">Local Operations</h2>
        <p class="text-muted">
          Run audit and fix operations directly on the current workspace.
        </p>
      </div>

      <div class="mb-4">
        <app-tabs
          [tabs]="tabs"
          [activeTabId]="activeTabId()"
          (tabChange)="onTabChange($event)"
        ></app-tabs>
      </div>

      <div class="operation-content">
        @if (activeTabId() === 'audit') {
          <app-local-audit (audit)="onAudit($event)"></app-local-audit>
        } @else {
          <app-local-fix (fix)="onFix($event)"></app-local-fix>
        }
      </div>
    </div>
  `,
  styles: `
    .mb-2 {
      margin-bottom: 8px;
    }
    .mb-4 {
      margin-bottom: 24px;
    }
    .text-muted {
      color: var(--color-fg-muted);
    }
  `,
})
export class LocalOpsPageComponent {
  /** Local ops service instance */
  private localOpService = inject(LocalOpService);
  /** Notification service instance */
  private notificationService = inject(NotificationService);

  /** Reference to audit component */
  @ViewChild(LocalAuditComponent) auditComponent!: LocalAuditComponent;
  /** Reference to fix component */
  @ViewChild(LocalFixComponent) fixComponent!: LocalFixComponent;

  /** Tabs for the page */
  tabs: TabItem[] = [
    { id: 'audit', label: 'Audit' },
    { id: 'fix', label: 'Fix' },
  ];

  /** Currently active tab */
  activeTabId = signal<string>('audit');

  /**
   * Handles tab changes.
   * @param tabId The selected tab ID
   */
  onTabChange(tabId: string): void {
    this.activeTabId.set(tabId);
    this.localOpService.clearResult();

    // Clear results from components
    if (this.auditComponent) this.auditComponent.setResult(null);
    if (this.fixComponent) this.fixComponent.setResult(null);
  }

  /**
   * Handles audit form submission.
   */
  onAudit(payload: {
    pattern: string;
    tools: string[];
    args: Record<string, unknown>;
  }): void {
    this.auditComponent.setOperating(true);
    this.localOpService
      .audit(payload.pattern, payload.tools, payload.args)
      .subscribe({
        next: (res: OpResult) => {
          this.auditComponent.setResult(res);
          this.auditComponent.setOperating(false);
        },
        error: () => {
          this.notificationService.error('Audit operation failed');
          this.auditComponent.setOperating(false);
        },
      });
  }

  /**
   * Handles fix form submission.
   */
  onFix(payload: {
    pattern: string;
    tools: string[];
    args: Record<string, unknown>;
    dryRun: boolean;
  }): void {
    this.fixComponent.setOperating(true);
    this.localOpService
      .fix(payload.pattern, payload.tools, payload.args, payload.dryRun)
      .subscribe({
        next: (res: OpResult) => {
          this.fixComponent.setResult(res);
          this.fixComponent.setOperating(false);
          if (payload.dryRun) {
            this.notificationService.info('Dry run completed');
          } else {
            this.notificationService.success('Fix operation completed');
          }
        },
        error: () => {
          this.notificationService.error('Fix operation failed');
          this.fixComponent.setOperating(false);
        },
      });
  }
}
