import { Component, input } from '@angular/core';

/**
 * A standard badge component following GitHub Primer styling.
 */
@Component({
  selector: 'app-badge',
  template: `
    <span class="Label" [class]="'Label--' + variant()">
      <ng-content></ng-content>
    </span>
  `,
  styles: `
    .Label {
      display: inline-block;
      padding: 0 7px;
      font-size: 12px;
      font-weight: 500;
      line-height: 18px;
      white-space: nowrap;
      border: 1px solid transparent;
      border-radius: 2em;
    }

    .Label--default {
      color: var(--color-fg-muted);
      border-color: var(--color-border-default);
    }

    .Label--success {
      color: var(--color-success-fg);
      border-color: var(--color-success-fg);
    }

    .Label--danger {
      color: var(--color-danger-fg);
      border-color: var(--color-danger-fg);
    }

    .Label--accent {
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
