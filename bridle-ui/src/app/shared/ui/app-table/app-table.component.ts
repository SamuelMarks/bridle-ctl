import { Component, ChangeDetectionStrategy, input, ContentChildren, QueryList, Directive, TemplateRef, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/**
 * Column definition for AppTable.
 */
@Directive({
  selector: '[appTableColumn]',
})
export class AppTableColumnDirective {
  /** The title of the column */
  title = input.required<string>();
  /** The key in the data object */
  key = input<string>();
  
  /** The template reference */
  public template = inject(TemplateRef<unknown>);
}

/**
 * A standard table component following GitHub Primer styling.
 */
@Component({
  selector: 'app-table',
  imports: [CommonModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="Box">
      @if (title()) {
        <div class="Box-header">
          <h3 class="Box-title">{{ title() }}</h3>
        </div>
      }
      <div class="Box-body p-0">
        <table class="table">
          <thead>
            <tr>
              @for (col of columns; track col.title()) {
                <th>{{ col.title() }}</th>
              }
            </tr>
          </thead>
          <tbody>
            @for (row of data(); track trackByFn()(row)) {
              <tr>
                @for (col of columns; track col.title()) {
                  <td>
                    <ng-container *ngTemplateOutlet="col.template; context: {$implicit: row, value: col.key() ? row[col.key()!] : null}"></ng-container>
                  </td>
                }
              </tr>
            }
            @if (data().length === 0 && emptyMessage()) {
              <tr>
                <td [attr.colspan]="columns.length" class="text-center text-muted p-3">
                  {{ emptyMessage() }}
                </td>
              </tr>
            }
          </tbody>
        </table>
      </div>
    </div>
  `,
  styles: `
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
      overflow: hidden;
    }
    
    .Box-header {
      padding: 16px;
      margin: -1px -1px 0;
      background-color: var(--color-canvas-subtle);
      border: 1px solid var(--color-border-default);
      border-top-left-radius: var(--border-radius-2);
      border-top-right-radius: var(--border-radius-2);
    }
    
    .Box-title {
      font-size: 14px;
      font-weight: 600;
      margin: 0;
    }
    
    .Box-body {
      padding: 16px;
      border-bottom: 1px solid var(--color-border-default);
    }
    
    .p-0 { padding: 0 !important; }
    .p-3 { padding: 16px !important; }
    .text-center { text-align: center !important; }
    .text-muted { color: var(--color-fg-muted) !important; }
    
    .table {
      width: 100%;
      border-collapse: collapse;
      text-align: left;
    }
    
    .table th, .table td {
      padding: 8px 16px;
      border-bottom: 1px solid var(--color-border-muted);
    }
    
    .table th {
      font-weight: 600;
      background-color: var(--color-canvas-subtle);
      color: var(--color-fg-muted);
      font-size: 12px;
    }
    
    .table tbody tr:last-child td {
      border-bottom: 0;
    }
    
    .table tbody tr:hover td {
      background-color: var(--color-canvas-subtle);
    }
  `
})
export class AppTableComponent {
  /** Optional title for the table box */
  title = input<string>('');
  
  /** Data to render */
  data = input<Record<string, unknown>[]>([]);
  
  /** Function to track rows */
  trackByFn = input<(item: unknown) => unknown>((item: unknown) => {
    if (item && typeof item === 'object' && 'id' in item) {
      return (item as Record<string, unknown>)['id'];
    }
    return item;
  });
  
  /** Message to show when data is empty */
  emptyMessage = input<string>('No items found.');

  /** List of column directives */
  @ContentChildren(AppTableColumnDirective) columns!: QueryList<AppTableColumnDirective>;
}
