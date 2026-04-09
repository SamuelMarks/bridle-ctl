import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from './api.service';
import { BatchJob } from '../models/models';
import { tap } from 'rxjs/operators';
import { Observable } from 'rxjs';

/**
 * Service for managing batch jobs.
 */
@Injectable({
  providedIn: 'root'
})
export class BatchService {
  /** API Service instance */
  private api = inject(ApiService);
  
  /** Signal holding job list */
  private jobsSignal = signal<BatchJob[]>([]);
  /** Signal holding active job */
  private activeJobSignal = signal<BatchJob | null>(null);

  /** List of all batch jobs */
  readonly jobs = this.jobsSignal.asReadonly();
  /** The currently viewed job */
  readonly activeJob = this.activeJobSignal.asReadonly();

  /**
   * Loads all batch jobs.
   */
  loadJobs(): Observable<BatchJob[]> {
    return this.api.get<BatchJob[]>('/batch/jobs').pipe(
      tap(jobs => this.jobsSignal.set(jobs))
    );
  }

  /**
   * Loads details for a specific job.
   * @param id Job ID
   */
  loadJob(id: string): Observable<BatchJob> {
    return this.api.get<BatchJob>(`/batch/jobs/${id}`).pipe(
      tap(job => this.activeJobSignal.set(job))
    );
  }

  /**
   * Creates a new batch fix job.
   * @param target Target organization or repo
   * @param title Title of the issue/PR
   * @param description Description of the issue/PR
   * @param pattern Regex pattern to match
   * @param tools Tools to run
   * @param args Arguments for the tools
   * @param safety_mode Whether to use safety mode
   * @param max_repos Maximum number of repositories to process
   * @param max_prs_per_hour Maximum number of PRs per hour
   */
  createBatchFix(target: string, title: string, description: string, pattern: string, tools: string[], args: Record<string, unknown>, safety_mode = true, max_repos?: number, max_prs_per_hour?: number): Observable<BatchJob> {
    return this.api.post<BatchJob>('/batch/fix', { org: target, issue: title, description, pattern, tools, tool_args: args, safety_mode, max_repos, max_prs_per_hour }).pipe(
      tap(job => this.jobsSignal.update(jobs => [job, ...jobs]))
    );
  }

  /**
   * Runs a pipeline from config.
   * @param config Pipeline configuration string
   * @param safety_mode Whether to use safety mode
   * @param max_repos Maximum number of repositories to process
   * @param max_prs_per_hour Maximum number of PRs per hour
   */
  runPipeline(config: string, safety_mode = true, max_repos?: number, max_prs_per_hour?: number): Observable<BatchJob> {
    return this.api.post<BatchJob>('/batch/run', { config_path: config, safety_mode, max_repos, max_prs_per_hour }).pipe(
      tap(job => this.jobsSignal.update(jobs => [job, ...jobs]))
    );
  }

  /**
   * Resumes an interrupted job.
   * @param id Job ID
   */
  resumeJob(id: string): Observable<BatchJob> {
    return this.api.post<BatchJob>(`/batch/jobs/${id}/resume`).pipe(
      tap(job => {
        this.activeJobSignal.set(job);
        this.jobsSignal.update(jobs => jobs.map(j => j.id === id ? job : j));
      })
    );
  }

  /**
   * Clears the currently active job.
   */
  clearActiveJob(): void {
    this.activeJobSignal.set(null);
  }
}
