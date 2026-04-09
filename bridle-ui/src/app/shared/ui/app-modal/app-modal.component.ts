import { Component, ChangeDetectionStrategy, input, output } from '@angular/core';

/**
 * A standard modal component following GitHub Primer styling.
 */
@Component({
  selector: 'app-modal',
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    @if (isOpen()) {
      <div class="modal-backdrop" tabindex="-1" (click)="closeOnBackdropClick() ? close.emit() : null" (keyup.escape)="close.emit()">
        <div class="Box modal-dialog" tabindex="-1" (click)="$event.stopPropagation()" (keyup.enter)="null">
          <div class="Box-header modal-header">
            <h3 class="Box-title">{{ title() }}</h3>
            <button type="button" class="close-button" (click)="close.emit()" aria-label="Close">
              <svg aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="octicon octicon-x">
                <path d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"></path>
              </svg>
            </button>
          </div>
          
          <div class="Box-body modal-body">
            <ng-content></ng-content>
          </div>
          
          @if (showFooter()) {
            <div class="Box-footer modal-footer">
              <ng-content select="[modal-footer]"></ng-content>
            </div>
          }
        </div>
      </div>
    }
  `,
  styles: `
    .modal-backdrop {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: rgba(27, 31, 36, 0.5);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 1000;
    }
    
    .Box {
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
      box-shadow: 0 8px 24px rgba(140, 149, 159, 0.2);
    }
    
    .modal-dialog {
      width: 100%;
      max-width: 448px;
      max-height: 80vh;
      display: flex;
      flex-direction: column;
    }
    
    .Box-header.modal-header {
      padding: 16px;
      background-color: var(--color-canvas-subtle);
      border-bottom: 1px solid var(--color-border-default);
      border-top-left-radius: var(--border-radius-2);
      border-top-right-radius: var(--border-radius-2);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    
    .Box-title {
      font-size: 14px;
      font-weight: 600;
      margin: 0;
    }
    
    .close-button {
      background: transparent;
      border: 0;
      cursor: pointer;
      color: var(--color-fg-muted);
      padding: 4px;
      display: flex;
    }
    
    .close-button:hover {
      color: var(--color-accent-fg);
    }
    
    .Box-body.modal-body {
      padding: 16px;
      overflow-y: auto;
    }
    
    .Box-footer.modal-footer {
      padding: 16px;
      border-top: 1px solid var(--color-border-default);
      background-color: var(--color-canvas-subtle);
      border-bottom-left-radius: var(--border-radius-2);
      border-bottom-right-radius: var(--border-radius-2);
      display: flex;
      justify-content: flex-end;
      gap: 8px;
    }
  `
})
export class AppModalComponent {
  /** Controls visibility of the modal */
  isOpen = input<boolean>(false);
  
  /** Title displayed in the modal header */
  title = input<string>('Modal Title');
  
  /** Whether clicking the backdrop should close the modal */
  closeOnBackdropClick = input<boolean>(true);
  
  /** Whether to show the footer area */
  showFooter = input<boolean>(true);
  
  /** Emitted when the modal requests to be closed */
  close = output<void>();
}
