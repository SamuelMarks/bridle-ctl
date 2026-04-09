import { Component, ChangeDetectionStrategy, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Organization } from '../../../core/models/models';
import { AppTableComponent, AppTableColumnDirective } from '../../../shared/ui/app-table/app-table.component';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';

/**
 * Component to display a list of ingested organizations.
 */
@Component({
  selector: 'app-org-list',
  imports: [CommonModule, AppTableComponent, AppTableColumnDirective, AppBadgeComponent],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <app-table title="Ingested Organizations" [data]="orgs()" emptyMessage="No organizations found.">
      <ng-template appTableColumn title="Name" let-org>
        <a href="javascript:void(0)" class="text-bold text-decoration-none color-fg-default" (click)="select.emit(org.id)">
          {{ org.name }}
        </a>
      </ng-template>
      
      <ng-template appTableColumn title="Provider" let-org>
        <app-badge [variant]="'default'">{{ org.provider }}</app-badge>
      </ng-template>

      <ng-template appTableColumn title="Actions" let-org>
        <a href="javascript:void(0)" class="text-muted" (click)="select.emit(org.id)">View Repositories</a>
      </ng-template>
    </app-table>
  `,
  styles: `
    .text-bold { font-weight: 600; }
    .text-decoration-none { text-decoration: none; }
    .color-fg-default { color: var(--color-fg-default); }
    .color-fg-default:hover { color: var(--color-accent-fg); text-decoration: underline; }
    .text-muted { color: var(--color-fg-muted); text-decoration: none; font-size: 12px; }
    .text-muted:hover { color: var(--color-accent-fg); }
  `
})
export class OrgListComponent {
  /** List of organizations to display */
  orgs = input<Organization[]>([]);

  /** Emitted when an organization is selected */
  select = output<string>();
}
