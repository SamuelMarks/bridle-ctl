import { Component, inject, OnInit } from '@angular/core';
import { SystemStateService } from '../../core/services/system-state.service';
import { AppButtonComponent } from '../../shared/ui/app-button/app-button.component';
import { AppBadgeComponent } from '../../shared/ui/app-badge/app-badge.component';

/**
 * Dashboard component displaying overall system health and quick actions.
 */
@Component({
  selector: 'app-dashboard',
  imports: [AppButtonComponent, AppBadgeComponent],
  template: `
    <div class="dashboard-container">
      <div class="page-header">
        <h1 class="h2">Dashboard</h1>
        <app-button
          variant="secondary"
          [disabled]="isLoading()"
          (click)="refreshHealth()"
        >
          {{ isLoading() ? 'Checking...' : 'Refresh Status' }}
        </app-button>
      </div>

      <div class="Box">
        <div class="Box-header">
          <h2 class="Box-title">System Health</h2>
        </div>
        <div class="Box-body health-grid">
          <div class="health-item">
            <span class="health-label">REST API</span>
            <app-badge
              [variant]="health().rest === 'UP' ? 'success' : 'danger'"
            >
              {{ health().rest }}
            </app-badge>
          </div>
          <div class="health-item">
            <span class="health-label">JSON-RPC Server</span>
            <app-badge [variant]="health().rpc === 'UP' ? 'success' : 'danger'">
              {{ health().rpc }}
            </app-badge>
          </div>
          <div class="health-item">
            <span class="health-label">Agent Daemon</span>
            <app-badge
              [variant]="health().agent === 'UP' ? 'success' : 'danger'"
            >
              {{ health().agent }}
            </app-badge>
          </div>
        </div>
      </div>
    </div>
  `,
  styles: `
    .dashboard-container {
      max-width: 1024px;
      margin: 0 auto;
    }

    .page-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
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

    .health-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 16px;
    }

    .health-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12px 16px;
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-1);
      background-color: var(--color-canvas-subtle);
    }

    .health-label {
      font-weight: 500;
      font-size: 14px;
    }
  `,
})
export class DashboardComponent implements OnInit {
  /** Injected SystemStateService */
  private systemState = inject(SystemStateService);

  /** Current system health signal */
  health = this.systemState.health;

  /** Whether health is loading */
  isLoading = this.systemState.isLoading;

  /** Initializes the component and fetches health */
  ngOnInit(): void {
    this.refreshHealth();
  }

  /** Refreshes health status */
  refreshHealth(): void {
    this.systemState.checkHealth().subscribe();
  }
}
