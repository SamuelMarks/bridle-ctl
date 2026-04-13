import {
  Component,
  ChangeDetectionStrategy,
  input,
  output,
} from '@angular/core';

/** Interface representing a single tab item */
export interface TabItem {
  /** The unique identifier for the tab */
  id: string;
  /** The label text to display */
  label: string;
  /** Optional badge number to display next to the label */
  badge?: number;
}

/**
 * A standard tabs component following GitHub Primer styling.
 */
@Component({
  selector: 'app-tabs',
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <nav class="UnderlineNav" aria-label="Tabs">
      <ul class="UnderlineNav-body" role="tablist">
        @for (tab of tabs(); track tab.id) {
          <li class="d-inline-flex" role="presentation">
            <button
              type="button"
              role="tab"
              class="UnderlineNav-item"
              [class.selected]="activeTabId() === tab.id"
              [attr.aria-selected]="activeTabId() === tab.id"
              (click)="selectTab(tab.id)"
            >
              <span>{{ tab.label }}</span>
              @if (tab.badge !== undefined) {
                <span class="Counter">{{ tab.badge }}</span>
              }
            </button>
          </li>
        }
      </ul>
    </nav>
  `,
  styles: `
    .UnderlineNav {
      display: flex;
      justify-content: space-between;
      border-bottom: 1px solid var(--color-border-default);
    }

    .UnderlineNav-body {
      display: flex;
      list-style: none;
      padding: 0;
      margin: 0;
      gap: 8px;
    }

    .UnderlineNav-item {
      padding: 8px 16px;
      font-size: 14px;
      line-height: 30px;
      color: var(--color-fg-default);
      text-align: center;
      background-color: transparent;
      border: 0;
      border-bottom: 2px solid transparent;
      cursor: pointer;
    }

    .UnderlineNav-item:hover {
      border-bottom-color: var(--color-border-muted);
      color: var(--color-fg-default);
      transition: border-bottom-color 0.12s ease-out;
    }

    .UnderlineNav-item.selected {
      font-weight: 600;
      color: var(--color-fg-default);
      border-bottom-color: #fd8c73; /* Primer active tab border color */
    }

    .Counter {
      display: inline-block;
      min-width: 20px;
      padding: 0 6px;
      font-size: 12px;
      font-weight: 500;
      line-height: 18px;
      color: var(--color-fg-default);
      text-align: center;
      background-color: var(--color-canvas-subtle);
      border-radius: 2em;
      margin-left: 8px;
    }
  `,
})
export class AppTabsComponent {
  /** Array of tabs to display */
  tabs = input<TabItem[]>([]);

  /** Currently active tab ID */
  activeTabId = input<string>('');

  /** Emitted when a tab is selected */
  tabChange = output<string>();

  /**
   * Selects a tab.
   * @param id The ID of the tab to select.
   */
  selectTab(id: string): void {
    this.tabChange.emit(id);
  }
}
