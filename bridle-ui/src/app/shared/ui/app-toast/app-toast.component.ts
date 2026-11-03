import { Component, inject } from '@angular/core';
import { NotificationService } from '../../../core/services/notification.service';

/**
 * Global toast notification component.
 */
@Component({
  selector: 'app-toast',
  template: `
    <div class="toast-container" aria-live="polite">
      @for (notification of notifications(); track notification.id) {
        <div class="Toast" [class]="'Toast--' + notification.type">
          <span class="Toast-content">{{ notification.message }}</span>
          <button
            type="button"
            class="Toast-dismissButton"
            (click)="dismiss(notification.id)"
            aria-label="Close"
          >
            &times;
          </button>
        </div>
      }
    </div>
  `,
  styles: `
    .toast-container {
      position: fixed;
      bottom: 24px;
      right: 24px;
      z-index: 9999;
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    .Toast {
      display: flex;
      align-items: center;
      justify-content: space-between;
      min-width: 280px;
      max-width: 400px;
      padding: 8px 16px;
      font-size: 14px;
      color: #ffffff;
      border-radius: var(--border-radius-1);
      box-shadow: var(--shadow-md);
    }

    .Toast--info {
      background-color: #0969da;
    }

    .Toast--success {
      background-color: #1a7f37;
    }

    .Toast--error {
      background-color: #cf222e;
    }

    .Toast-content {
      padding-right: 12px;
    }

    .Toast-dismissButton {
      background: none;
      border: none;
      color: inherit;
      font-size: 18px;
      line-height: 1;
      cursor: pointer;
      opacity: 0.8;
      padding: 4px;
    }

    .Toast-dismissButton:hover {
      opacity: 1;
    }
  `,
})
export class AppToastComponent {
  /** Injected NotificationService */
  private notificationService = inject(NotificationService);

  /** Signal of current notifications */
  notifications = this.notificationService.notifications;

  /**
   * Dismisses a notification by its unique ID.
   * @param id The unique identifier of the notification to dismiss.
   */
  dismiss(id: string): void {
    this.notificationService.remove(id);
  }
}
