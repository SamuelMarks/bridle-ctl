import { Component, inject, viewChild, signal } from '@angular/core';
import {
  LocalOpService,
  OpResult,
} from '../../../core/services/local-op.service';
import { NotificationService } from '../../../core/services/notification.service';
import { JsonObject } from '../../../core/models/models';
import {
  AppTabsComponent,
  TabItem,
} from '../../../shared/ui/app-tabs/app-tabs.component';
import { LocalAuditComponent } from '../local-audit/local-audit.component';
import { LocalFixComponent } from '../local-fix/local-fix.component';

/**
 * Page component for executing local audit and fix operations.
 */
@Component({
  selector: 'app-local-ops-page',
  imports: [AppTabsComponent, LocalAuditComponent, LocalFixComponent],
  template: `
    <div class="local-ops-container">
      <div class="page-header">
        <h1 class="h2">Local Operations</h1>
      </div>

      <app-tabs
        [tabs]="tabs"
        [activeTab]="activeTab()"
        (activeTabChange)="onTabChange($event)"
      >
        @if (activeTab() === 'audit') {
          <app-local-audit (audit)="onAudit($event)"></app-local-audit>
        } @else {
          <app-local-fix (fix)="onFix($event)"></app-local-fix>
        }
      </app-tabs>
    </div>
  `,
  styles: `
    .local-ops-container {
      max-width: 1024px;
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
  `,
})
export class LocalOpsPageComponent {
  /** Injected LocalOpService */
  private localOpService = inject(LocalOpService);
  /** Injected NotificationService */
  private notificationService = inject(NotificationService);

  /** Signal query for LocalAuditComponent */
  readonly auditComponent = viewChild(LocalAuditComponent);

  /** Signal query for LocalFixComponent */
  readonly fixComponent = viewChild(LocalFixComponent);

  /** Available tabs */
  tabs: TabItem[] = [
    { id: 'audit', label: 'Audit Codebase' },
    { id: 'fix', label: 'Fix Issues' },
  ];

  /** Currently active tab */
  activeTab = signal('audit');

  /**
   * Handles tab change.
   * @param tabId The newly selected tab ID
   */
  onTabChange(tabId: string): void {
    this.activeTab.set(tabId);
    this.localOpService.clearResult();
  }

  /**
   * Handles running an audit.
   * @param data The audit parameters
   * @param data.pattern The target regex pattern
   * @param data.tools The tools to run
   * @param data.args The tool arguments
   */
  onAudit(data: { pattern: string; tools: string[]; args: JsonObject }): void {
    this.auditComponent()?.setOperating(true);
    this.localOpService.audit(data.pattern, data.tools, data.args).subscribe({
      next: (res: OpResult) => {
        this.auditComponent()?.setResult(res);
        this.auditComponent()?.setOperating(false);
        this.notificationService.success('Audit completed');
      },
      error: () => {
        this.notificationService.error('Audit failed');
        this.auditComponent()?.setOperating(false);
      },
    });
  }

  /**
   * Handles running a fix.
   * @param data The fix parameters
   * @param data.pattern The target regex pattern
   * @param data.tools The tools to run
   * @param data.args The tool arguments
   * @param data.dryRun Whether to run in dry-run mode
   */
  onFix(data: {
    pattern: string;
    tools: string[];
    args: JsonObject;
    dryRun: boolean;
  }): void {
    this.fixComponent()?.setOperating(true);
    this.localOpService
      .fix(data.pattern, data.tools, data.args, data.dryRun)
      .subscribe({
        next: (res: OpResult) => {
          this.fixComponent()?.setResult(res);
          this.fixComponent()?.setOperating(false);
          this.notificationService.success('Fix operation completed');
        },
        error: () => {
          this.notificationService.error('Fix operation failed');
          this.fixComponent()?.setOperating(false);
        },
      });
  }
}
