import { Component, ChangeDetectionStrategy, inject } from '@angular/core';

import { NotificationService } from '../../../core/services/notification.service';

/**
 * Global toast notification component.
 */
@Component({
  selector: 'app-toast',
  imports: [],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div
      class="toast-container"
      role="region"
      aria-label="Notifications"
      aria-live="polite"
    >
      @for (notification of notifications(); track notification.id) {
        <div
          class="Toast"
          [class.Toast--success]="notification.type === 'success'"
          [class.Toast--error]="notification.type === 'error'"
          [class.Toast--info]="notification.type === 'info'"
        >
          <span class="Toast-icon">
            @if (notification.type === 'success') {
              <svg
                aria-hidden="true"
                height="16"
                viewBox="0 0 16 16"
                version="1.1"
                width="16"
                data-view-component="true"
                class="octicon octicon-check"
              >
                <path
                  d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"
                ></path>
              </svg>
            } @else if (notification.type === 'error') {
              <svg
                aria-hidden="true"
                height="16"
                viewBox="0 0 16 16"
                version="1.1"
                width="16"
                data-view-component="true"
                class="octicon octicon-stop"
              >
                <path
                  d="M4.47.22A.749.749 0 0 1 5 0h6c.199 0 .389.079.53.22l4.25 4.25c.141.14.22.331.22.53v6a.749.749 0 0 1-.22.53l-4.25 4.25A.749.749 0 0 1 11 16H5a.749.749 0 0 1-.53-.22L.22 11.53A.749.749 0 0 1 0 11V5c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5ZM8 4a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"
                ></path>
              </svg>
            } @else {
              <svg
                aria-hidden="true"
                height="16"
                viewBox="0 0 16 16"
                version="1.1"
                width="16"
                data-view-component="true"
                class="octicon octicon-info"
              >
                <path
                  d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"
                ></path>
              </svg>
            }
          </span>
          <span class="Toast-content">{{ notification.message }}</span>
          <button
            class="Toast-dismissButton"
            aria-label="Dismiss notification"
            (click)="dismiss(notification.id)"
          >
            <svg
              aria-hidden="true"
              height="16"
              viewBox="0 0 16 16"
              version="1.1"
              width="16"
              data-view-component="true"
              class="octicon octicon-x"
            >
              <path
                d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
              ></path>
            </svg>
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
      display: flex;
      flex-direction: column;
      gap: 8px;
      z-index: 100;
    }

    .Toast {
      display: flex;
      align-items: center;
      padding: 16px;
      color: var(--color-fg-default);
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-2);
      box-shadow: 0 8px 24px rgba(140, 149, 159, 0.2);
      min-width: 300px;
    }

    .Toast--success {
      border-color: var(--color-success-emphasis);
    }
    .Toast--success .Toast-icon {
      color: var(--color-success-fg);
    }

    .Toast--error {
      border-color: var(--color-danger-emphasis);
    }
    .Toast--error .Toast-icon {
      color: var(--color-danger-fg);
    }

    .Toast--info {
      border-color: var(--color-accent-emphasis);
    }
    .Toast--info .Toast-icon {
      color: var(--color-accent-fg);
    }

    .Toast-icon {
      margin-right: 12px;
      display: flex;
    }

    .Toast-content {
      flex: 1;
      font-size: 14px;
    }

    .Toast-dismissButton {
      background: transparent;
      border: 0;
      padding: 0;
      margin-left: 12px;
      color: var(--color-fg-muted);
      cursor: pointer;
      display: flex;
    }

    .Toast-dismissButton:hover {
      color: var(--color-fg-default);
    }
  `,
})
export class AppToastComponent {
  /**
   * Private reference to the notification service.
   */
  private notificationService = inject(NotificationService);

  /**
   * The list of active notifications to display.
   */
  notifications = this.notificationService.notifications;

  /**
   * Dismisses a notification by its unique ID.
   * @param id The unique identifier of the notification to dismiss.
   */
  dismiss(id: string): void {
    this.notificationService.remove(id);
  }
}
