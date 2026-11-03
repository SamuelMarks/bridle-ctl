import { Component, input } from '@angular/core';

/**
 * A standard button component following GitHub Primer styling.
 */
@Component({
  selector: 'app-button',
  template: `
    <button
      [type]="type()"
      [disabled]="disabled()"
      [attr.aria-disabled]="disabled()"
      class="btn"
      [class]="'btn-' + variant()"
    >
      <ng-content></ng-content>
    </button>
  `,
  styles: `
    .btn {
      position: relative;
      display: inline-block;
      padding: 5px 16px;
      font-size: 14px;
      font-weight: 500;
      line-height: 20px;
      white-space: nowrap;
      vertical-align: middle;
      cursor: pointer;
      user-select: none;
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius);
      appearance: none;
      transition: 80ms cubic-bezier(0.33, 1, 0.68, 1);
      transition-property: color, background-color, box-shadow, border-color;
    }

    .btn:disabled {
      cursor: default;
      opacity: 0.6;
      pointer-events: none;
    }

    .btn-secondary {
      color: var(--color-fg-default);
      background-color: var(--color-canvas-subtle);
      border-color: var(--color-border-default);
      box-shadow: var(--shadow-sm);
    }

    .btn-secondary:hover:not(:disabled) {
      background-color: #f3f4f6;
      border-color: var(--color-border-default);
    }

    .btn-primary {
      color: #ffffff;
      background-color: var(--color-btn-primary-bg, #1a7f37);
      border-color: rgba(31, 35, 40, 0.15);
      box-shadow: var(--shadow-sm);
    }

    .btn-primary:hover:not(:disabled) {
      background-color: var(--color-btn-primary-hover-bg, #116023);
    }

    .btn-danger {
      color: var(--color-danger-fg);
      background-color: var(--color-canvas-subtle);
      border-color: var(--color-border-default);
      box-shadow: var(--shadow-sm);
    }

    .btn-danger:hover:not(:disabled) {
      color: #ffffff;
      background-color: var(--color-danger-fg);
      border-color: rgba(31, 35, 40, 0.15);
    }

    .btn-invisible {
      color: var(--color-accent-fg);
      background-color: transparent;
      border-color: transparent;
      box-shadow: none;
    }

    .btn-invisible:hover:not(:disabled) {
      background-color: var(--color-canvas-subtle);
    }
  `,
})
export class AppButtonComponent {
  /**
   * Visual variant of the button.
   */
  variant = input<'primary' | 'secondary' | 'danger' | 'invisible'>(
    'secondary',
  );

  /**
   * HTML button type.
   */
  type = input<'button' | 'submit' | 'reset'>('button');

  /**
   * Whether the button is disabled.
   */
  disabled = input<boolean>(false);
}
