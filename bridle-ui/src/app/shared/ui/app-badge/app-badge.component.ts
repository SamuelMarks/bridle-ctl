import { Component, ChangeDetectionStrategy, input } from '@angular/core';

/**
 * A standard badge component following GitHub Primer styling.
 */
@Component({
  selector: 'app-badge',
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <span class="badge" [class]="'badge-' + variant()">
      <ng-content></ng-content>
    </span>
  `,
  styles: `
    .badge {
      display: inline-block;
      padding: 0 7px;
      font-size: 12px;
      font-weight: 500;
      line-height: 18px;
      white-space: nowrap;
      border: 1px solid transparent;
      border-radius: 2em;
    }

    .badge-default {
      color: var(--color-fg-muted);
      border-color: var(--color-border-default);
    }

    .badge-success {
      color: var(--color-success-fg);
      border-color: var(--color-success-fg);
    }

    .badge-danger {
      color: var(--color-danger-fg);
      border-color: var(--color-danger-fg);
    }

    .badge-accent {
      color: var(--color-accent-fg);
      border-color: var(--color-accent-fg);
    }
  `,
})
export class AppBadgeComponent {
  /**
   * Visual variant of the badge.
   */
  variant = input<'default' | 'success' | 'danger' | 'accent'>('default');
}
