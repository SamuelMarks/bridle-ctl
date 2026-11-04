import { Service, inject } from '@angular/core';
import { ApiService } from './api.service';
import { BatchJob, JsonObject } from '../models/models';
import { Observable } from 'rxjs';

/**
 * Stateless HTTP data-access service for batch operations and pipelines.
 * Reactive state is managed by `JobsStore`.
 */
@Service()
export class BatchService {
  /** Injected API service */
  private api = inject(ApiService);

  /**
   * Loads all batch jobs.
   * @returns Observable of BatchJob array
   */
  loadJobs(): Observable<BatchJob[]> {
    return this.api.get<BatchJob[]>('/batch/jobs');
  }

  /**
   * Loads a specific batch job by ID.
   * @param id The job ID
   * @returns Observable of BatchJob
   */
  loadJob(id: string): Observable<BatchJob> {
    return this.api.get<BatchJob>(`/batch/jobs/${id}`);
  }

  /**
   * Creates a new batch fix job.
   * @param orgId Target organization ID
   * @param title Issue title
   * @param description Issue description
   * @param pattern Target regex pattern
   * @param tools Tools to run
   * @param args Tool arguments
   * @param safetyMode Whether to run in safety mode
   * @param maxRepos Maximum number of repositories to process
   * @param maxPrsPerHour Global limit of PRs per hour
   * @returns Observable of created BatchJob
   */
  createBatchFix(
    orgId: string,
    title: string,
    description: string,
    pattern: string,
    tools: string[],
    args: JsonObject,
    safetyMode = true,
    maxRepos?: number,
    maxPrsPerHour?: number,
  ): Observable<BatchJob> {
    return this.api.post<BatchJob>('/batch/fix', {
      org: orgId,
      issue: title,
      description,
      pattern,
      tools,
      tool_args: args,
      safety_mode: safetyMode,
      max_repos: maxRepos,
      max_prs_per_hour: maxPrsPerHour,
    });
  }

  /**
   * Runs a batch pipeline from YAML configuration.
   * @param configYaml The YAML configuration string
   * @param safetyMode Whether to run in safety mode
   * @param maxRepos Maximum number of repositories to process
   * @param maxPrsPerHour Global limit of PRs per hour
   * @returns Observable of created BatchJob
   */
  runPipeline(
    configYaml: string,
    safetyMode = true,
    maxRepos?: number,
    maxPrsPerHour?: number,
  ): Observable<BatchJob> {
    return this.api.post<BatchJob>('/batch/run', {
      config_path: configYaml,
      safety_mode: safetyMode,
      max_repos: maxRepos,
      max_prs_per_hour: maxPrsPerHour,
    });
  }

  /**
   * Resumes an interrupted batch job.
   * @param id The job ID
   * @returns Observable of resumed BatchJob
   */
  resumeJob(id: string): Observable<BatchJob> {
    return this.api.post<BatchJob>(`/batch/jobs/${id}/resume`);
  }
}
