import { Injectable, signal } from '@angular/core';

/** Notification interface */
export interface Notification {
  /** The unique id */
  id: string;
  /** The type of notification */
  type: 'success' | 'error' | 'info';
  /** The text message */
  message: string;
}

/**
 * Service for managing toast notifications.
 */
@Injectable({
  providedIn: 'root'
})
export class NotificationService {
  /** Signal for notifications */
  private notificationsSignal = signal<Notification[]>([]);
  
  /** Current notifications */
  readonly notifications = this.notificationsSignal.asReadonly();

  /**
   * Show a success notification.
   * @param message The message to display
   */
  success(message: string): void {
    this.addNotification('success', message);
  }

  /**
   * Show an error notification.
   * @param message The message to display
   */
  error(message: string): void {
    this.addNotification('error', message);
  }

  /**
   * Show an info notification.
   * @param message The message to display
   */
  info(message: string): void {
    this.addNotification('info', message);
  }

  /**
   * Remove a notification by ID.
   * @param id The notification ID
   */
  remove(id: string): void {
    this.notificationsSignal.update(notifications => 
      notifications.filter(n => n.id !== id)
    );
  }

  /**
   * Internal method to add a notification.
   * @param type Notification type
   * @param message Notification message
   */
  private addNotification(type: 'success' | 'error' | 'info', message: string): void {
    const id = Math.random().toString(36).substring(2, 9);
    this.notificationsSignal.update(notifications => [
      ...notifications,
      { id, type, message }
    ]);

    // Auto-remove after 5 seconds
    setTimeout(() => {
      this.remove(id);
    }, 5000);
  }
}
