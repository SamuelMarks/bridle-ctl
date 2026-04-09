import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from './api.service';
import { SystemHealth } from '../models/models';
import { tap } from 'rxjs/operators';

/**
 * Service for managing system health state.
 */
@Injectable({
  providedIn: 'root'
})
export class SystemStateService {
  /** API Service instance */
  private api = inject(ApiService);
  
  /** Signal for health */
  private healthSignal = signal<SystemHealth>({ rest: 'DOWN', rpc: 'DOWN', agent: 'DOWN' });
  /** Signal for loading state */
  private isLoadingSignal = signal<boolean>(false);

  /** Current health state */
  readonly health = this.healthSignal.asReadonly();
  /** Whether a check is currently in progress */
  readonly isLoading = this.isLoadingSignal.asReadonly();

  /**
   * Fetches the current system health.
   */
  checkHealth() {
    this.isLoadingSignal.set(true);
    return this.api.get<SystemHealth>('/health').pipe(
      tap({
        next: (health) => {
          this.healthSignal.set(health);
          this.isLoadingSignal.set(false);
        },
        error: () => {
          this.healthSignal.set({ rest: 'DOWN', rpc: 'DOWN', agent: 'DOWN' });
          this.isLoadingSignal.set(false);
        }
      })
    );
  }
}
