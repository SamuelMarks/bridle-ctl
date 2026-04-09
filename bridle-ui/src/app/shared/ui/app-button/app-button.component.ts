import { Component, ChangeDetectionStrategy, input } from '@angular/core';

/**
 * A standard button component following GitHub Primer styling.
 */
@Component({
  selector: 'app-button',
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <button
      [class]="'btn btn-' + variant()"
      [disabled]="disabled()"
      [type]="type()"
      [attr.aria-disabled]="disabled()"
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
      border: 1px solid;
      border-radius: var(--border-radius-1);
      appearance: none;
    }
    
    .btn:disabled {
      cursor: not-allowed;
      opacity: 0.6;
    }

    .btn-secondary {
      color: var(--color-btn-text);
      background-color: var(--color-btn-bg);
      border-color: var(--color-btn-border);
    }
    
    .btn-secondary:hover:not(:disabled) {
      background-color: var(--color-btn-hover-bg);
      border-color: var(--color-btn-hover-border);
    }

    .btn-primary {
      color: var(--color-btn-primary-text);
      background-color: var(--color-btn-primary-bg);
      border-color: var(--color-btn-primary-border);
    }
    
    .btn-primary:hover:not(:disabled) {
      background-color: var(--color-btn-primary-hover-bg);
    }

    .btn-danger {
      color: var(--color-btn-danger-text);
      background-color: var(--color-btn-danger-bg);
      border-color: var(--color-btn-border);
    }
    
    .btn-danger:hover:not(:disabled) {
      color: var(--color-btn-danger-hover-text);
      background-color: var(--color-btn-danger-hover-bg);
      border-color: var(--color-btn-danger-hover-bg);
    }

    .btn-invisible {
      color: var(--color-accent-fg);
      background-color: transparent;
      border-color: transparent;
    }
    
    .btn-invisible:hover:not(:disabled) {
      color: var(--color-accent-emphasis);
    }
  `
})
export class AppButtonComponent {
  /**
   * Visual variant of the button.
   */
  variant = input<'primary' | 'secondary' | 'danger' | 'invisible'>('secondary');

  /**
   * HTML button type.
   */
  type = input<'button' | 'submit' | 'reset'>('button');

  /**
   * Whether the button is disabled.
   */
  disabled = input<boolean>(false);
}
