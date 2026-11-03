import { Component, input } from '@angular/core';
import { Repository } from '../../../core/models/models';
import {
  AppTableComponent,
  AppTableColumnDirective,
} from '../../../shared/ui/app-table/app-table.component';

/**
 * Component displaying a table of repositories for a selected organization.
 */
@Component({
  selector: 'app-repo-list',
  imports: [AppTableComponent, AppTableColumnDirective],
  template: `
    <app-table
      [title]="'Repositories for ' + orgName()"
      [data]="repos()"
      emptyMessage="No repositories found for this organization."
    >
      <ng-template appTableColumn title="Repository" key="name" let-repo>
        <div class="repo-name">
          <svg
            aria-hidden="true"
            height="16"
            viewBox="0 0 16 16"
            version="1.1"
            width="16"
            class="octicon octicon-repo text-muted mr-2"
          >
            <path
              d="M2 2.5A2.5 2.5 0 0 1 4.5 0h8.75a.75.75 0 0 1 .75.75v12.5a.75.75 0 0 1-.75.75h-2.5a.75.75 0 0 1 0-1.5h1.75v-2h-8a1 1 0 0 0-.714 1.7.75.75 0 1 1-1.072 1.05A2.495 2.495 0 0 1 2 11.5Zm10.5-1h-8a1 1 0 0 0-1 1v6.708A2.486 2.486 0 0 1 4.5 9h8ZM5 12.25a.25.25 0 0 1 .25-.25h3.5a.25.25 0 0 1 .25.25v3.25a.25.25 0 0 1-.4.2l-1.45-1.087a.249.249 0 0 0-.3 0L5.4 15.7a.25.25 0 0 1-.4-.2Z"
            ></path>
          </svg>
          <strong>{{ repo.name }}</strong>
        </div>
      </ng-template>
      <ng-template
        appTableColumn
        title="Description"
        key="description"
        let-value="value"
      >
        <span class="text-muted">{{ value || 'No description' }}</span>
      </ng-template>
    </app-table>
  `,
  styles: `
    .repo-name {
      display: flex;
      align-items: center;
      gap: 8px;
    }
    .text-muted {
      color: var(--color-fg-muted);
    }
    .octicon {
      fill: currentColor;
    }
  `,
})
export class RepoListComponent {
  /** The name of the organization to display in the title */
  orgName = input<string>('');

  /** List of repositories to display */
  repos = input<Repository[]>([]);
}
