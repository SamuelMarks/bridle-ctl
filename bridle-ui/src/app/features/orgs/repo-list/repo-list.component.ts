import { Component, ChangeDetectionStrategy, input } from '@angular/core';

import { Repository } from '../../../core/models/models';
import {
  AppTableComponent,
  AppTableColumnDirective,
} from '../../../shared/ui/app-table/app-table.component';

/**
 * Component to display a list of repositories.
 */
@Component({
  selector: 'app-repo-list',
  imports: [AppTableComponent, AppTableColumnDirective],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <app-table
      [title]="'Repositories for ' + orgName()"
      [data]="repos()"
      emptyMessage="No repositories found."
    >
      <ng-template appTableColumn title="Repository Name" let-repo>
        <div class="d-flex flex-column">
          <span class="text-bold color-fg-default">{{ repo.name }}</span>
          @if (repo.description) {
            <span class="text-muted text-small mt-1">{{
              repo.description
            }}</span>
          }
        </div>
      </ng-template>

      <ng-template appTableColumn title="Link" let-repo>
        @if (repo.url) {
          <a
            [href]="repo.url"
            target="_blank"
            rel="noopener noreferrer"
            class="text-muted text-small"
          >
            <svg
              aria-hidden="true"
              height="16"
              viewBox="0 0 16 16"
              version="1.1"
              width="16"
              class="octicon octicon-link-external"
            >
              <path
                d="M3.75 2h3.5a.75.75 0 0 1 0 1.5h-3.5a.25.25 0 0 0-.25.25v8.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25v-3.5a.75.75 0 0 1 1.5 0v3.5A1.75 1.75 0 0 1 12.25 14h-8.5A1.75 1.75 0 0 1 2 12.25v-8.5C2 2.784 2.784 2 3.75 2Zm6.854-1h4.146a.25.25 0 0 1 .25.25v4.146a.25.25 0 0 1-.427.177L13.03 4.03 9.28 7.78a.75.75 0 0 1-1.06-1.06l3.75-3.75-1.543-1.543A.25.25 0 0 1 10.604 1Z"
              ></path>
            </svg>
            Open
          </a>
        }
      </ng-template>
    </app-table>
  `,
  styles: `
    .text-bold {
      font-weight: 600;
    }
    .color-fg-default {
      color: var(--color-fg-default);
    }
    .text-muted {
      color: var(--color-fg-muted);
      text-decoration: none;
    }
    .text-muted:hover {
      color: var(--color-accent-fg);
      text-decoration: underline;
    }
    .text-small {
      font-size: 12px;
    }
    .mt-1 {
      margin-top: 4px;
    }
    .octicon {
      vertical-align: text-bottom;
      margin-right: 4px;
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
