import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from './api.service';
import { Observable } from 'rxjs';
import { tap } from 'rxjs/operators';

/** Result of a local operation */
export interface OpResult {
  /** Output string */
  output: string;
  /** Diff string */
  diff?: string;
  /** List of modified files */
  modifiedFiles?: string[];
}

/**
 * Service for local audit and fix operations.
 */
@Injectable({
  providedIn: 'root',
})
export class LocalOpService {
  /** API Service instance */
  private api = inject(ApiService);

  /** Signal for operating state */
  private isOperatingSignal = signal<boolean>(false);
  /** Signal for last result */
  private lastResultSignal = signal<OpResult | null>(null);

  /** Whether an operation is in progress */
  readonly isOperating = this.isOperatingSignal.asReadonly();
  /** Result of the last operation */
  readonly lastResult = this.lastResultSignal.asReadonly();

  /**
   * Executes an audit operation.
   * @param pattern Regex pattern to audit
   * @param tools Selected tools
   * @param args Tool arguments
   */
  audit(
    pattern: string,
    tools: string[],
    args: Record<string, unknown>,
  ): Observable<OpResult> {
    this.isOperatingSignal.set(true);
    return this.api
      .post<OpResult>('/local/audit', { pattern, tools, args })
      .pipe(
        tap({
          next: (res) => {
            this.lastResultSignal.set(res);
            this.isOperatingSignal.set(false);
          },
          error: () => this.isOperatingSignal.set(false),
        }),
      );
  }

  /**
   * Executes a fix operation.
   * @param pattern Regex pattern to fix
   * @param tools Selected tools
   * @param args Tool arguments
   * @param dryRun Whether to perform a dry run
   */
  fix(
    pattern: string,
    tools: string[],
    args: Record<string, unknown>,
    dryRun: boolean,
  ): Observable<OpResult> {
    this.isOperatingSignal.set(true);
    return this.api
      .post<OpResult>('/local/fix', { pattern, tools, args, dryRun })
      .pipe(
        tap({
          next: (res) => {
            this.lastResultSignal.set(res);
            this.isOperatingSignal.set(false);
          },
          error: () => this.isOperatingSignal.set(false),
        }),
      );
  }

  /**
   * Clears the last result.
   */
  clearResult(): void {
    this.lastResultSignal.set(null);
  }
}
