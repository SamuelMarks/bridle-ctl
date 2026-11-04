import { Component, input, output } from '@angular/core';
import { Organization } from '../../../core/models/models';
import {
  AppTableComponent,
  AppTableColumnDirective,
} from '../../../shared/ui/app-table/app-table.component';
import { AppBadgeComponent } from '../../../shared/ui/app-badge/app-badge.component';

/**
 * Component displaying a table of organizations.
 */
@Component({
  selector: 'app-org-list',
  imports: [AppTableComponent, AppTableColumnDirective, AppBadgeComponent],
  template: `
    <app-table
      title="Organizations"
      [data]="orgs()"
      emptyMessage="No organizations ingested yet."
    >
      <ng-template appTableColumn title="Name" key="name" let-org>
        <button type="button" class="org-link" (click)="select.emit(org.id)">
          <strong>{{ org.name }}</strong>
        </button>
      </ng-template>
      <ng-template appTableColumn title="Provider" key="provider" let-org>
        <app-badge variant="default">{{ org.provider }}</app-badge>
      </ng-template>
      <ng-template appTableColumn title="Actions" let-org>
        <button type="button" class="btn-link" (click)="select.emit(org.id)">
          View Repos
        </button>
      </ng-template>
    </app-table>
  `,
  styles: `
    .org-link {
      background: none;
      border: 0;
      padding: 0;
      font: inherit;
      color: var(--color-fg-default);
      cursor: pointer;
      text-decoration: none;
    }
    .org-link:hover {
      color: var(--color-accent-fg);
      text-decoration: underline;
    }
    .btn-link {
      background: none;
      border: 0;
      padding: 0;
      cursor: pointer;
      font-size: 12px;
      font-weight: 500;
      color: var(--color-accent-fg);
    }
    .btn-link:hover {
      text-decoration: underline;
    }
  `,
})
export class OrgListComponent {
  /** List of organizations to display */
  orgs = input<Organization[]>([]);

  /** Emitted when an organization is selected */
  select = output<string>();
}
