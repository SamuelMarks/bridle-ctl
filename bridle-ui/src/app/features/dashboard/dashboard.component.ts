import {
  Component,
  ChangeDetectionStrategy,
  inject,
  OnInit,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { SystemStateService } from '../../core/services/system-state.service';
import { AppButtonComponent } from '../../shared/ui/app-button/app-button.component';
import { AppBadgeComponent } from '../../shared/ui/app-badge/app-badge.component';

/**
 * Dashboard Component showing system health.
 */
@Component({
  selector: 'app-dashboard',
  imports: [CommonModule, AppButtonComponent, AppBadgeComponent],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="container-lg">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>System Health Dashboard</h2>
        <app-button
          variant="primary"
          [disabled]="isLoading()"
          (click)="refreshHealth()"
        >
          Refresh Health Check
        </app-button>
      </div>

      <div class="health-cards">
        <div class="Box p-3">
          <h3 class="h4 mb-2">REST API</h3>
          <p class="text-muted">Status of the HTTP interface.</p>
          <app-badge [variant]="health().rest === 'UP' ? 'success' : 'danger'">
            {{ health().rest }}
          </app-badge>
        </div>

        <div class="Box p-3">
          <h3 class="h4 mb-2">RPC Server</h3>
          <p class="text-muted">Status of the internal RPC channel.</p>
          <app-badge [variant]="health().rpc === 'UP' ? 'success' : 'danger'">
            {{ health().rpc }}
          </app-badge>
        </div>

        <div class="Box p-3">
          <h3 class="h4 mb-2">Agent Daemon</h3>
          <p class="text-muted">Status of the background execution agent.</p>
          <app-badge [variant]="health().agent === 'UP' ? 'success' : 'danger'">
            {{ health().agent }}
          </app-badge>
        </div>
      </div>
    </div>
  `,
  styles: `
    .mb-4 {
      margin-bottom: 24px;
    }
    .mb-2 {
      margin-bottom: 8px;
    }
    .p-3 {
      padding: 16px;
    }
    .h4 {
      font-size: 16px;
      font-weight: 600;
      margin-top: 0;
    }

    .health-cards {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 16px;
    }

    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
    }
  `,
})
export class DashboardComponent implements OnInit {
  /** System state service instance */
  private systemState = inject(SystemStateService);

  /** Current system health */
  health = this.systemState.health;
  /** Whether loading */
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
