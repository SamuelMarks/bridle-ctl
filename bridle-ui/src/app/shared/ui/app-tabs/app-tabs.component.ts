import { Component, input, model } from '@angular/core';

/** Interface representing a single tab item */
export interface TabItem {
  /** The unique identifier for the tab */
  id: string;
  /** The display label for the tab */
  label: string;
}

/**
 * A standard tabs component following GitHub Primer underline nav styling.
 */
@Component({
  selector: 'app-tabs',
  template: `
    <nav class="UnderlineNav" aria-label="Tabs">
      <div class="UnderlineNav-body" role="tablist">
        @for (tab of tabs(); track tab.id) {
          <button
            type="button"
            class="UnderlineNav-item"
            role="tab"
            [attr.aria-selected]="activeTab() === tab.id"
            [class.selected]="activeTab() === tab.id"
            (click)="selectTab(tab.id)"
          >
            {{ tab.label }}
          </button>
        }
      </div>
    </nav>
    <div class="tab-content" role="tabpanel">
      <ng-content></ng-content>
    </div>
  `,
  styles: `
    .UnderlineNav {
      display: flex;
      overflow-x: auto;
      overflow-y: hidden;
      border-bottom: 1px solid var(--color-border-default);
      justify-content: space-between;
      margin-bottom: 16px;
    }

    .UnderlineNav-body {
      display: flex;
      align-items: center;
      gap: 8px;
    }

    .UnderlineNav-item {
      padding: 8px 16px;
      font-size: 14px;
      line-height: 30px;
      color: var(--color-fg-default);
      text-align: center;
      white-space: nowrap;
      background-color: transparent;
      border: 0;
      border-bottom: 2px solid transparent;
      cursor: pointer;
      transition: border-bottom-color 0.2s ease-in;
    }

    .UnderlineNav-item:hover {
      color: var(--color-fg-default);
      text-decoration: none;
      border-bottom-color: var(--color-border-muted);
    }

    .UnderlineNav-item.selected {
      font-weight: 600;
      color: var(--color-fg-default);
      border-bottom-color: #fd8c73;
    }

    .tab-content {
      padding-top: 8px;
    }
  `,
})
export class AppTabsComponent {
  /** Array of tabs to display */
  tabs = input<TabItem[]>([]);

  /** Currently active tab ID */
  activeTab = model<string>('');

  /**
   * Selects a tab.
   * @param id The ID of the tab to select.
   */
  selectTab(id: string): void {
    this.activeTab.set(id);
  }
}
