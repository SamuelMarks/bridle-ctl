/**
 * Organization model representing a group of repositories.
 */
export interface Organization {
  /** Unique identifier */
  id: string;
  /** Organization name */
  name: string;
  /** Source provider (e.g., 'github') */
  provider: string;
}

/**
 * Repository model.
 */
export interface Repository {
  /** Unique identifier */
  id: string;
  /** Repository name */
  name: string;
  /** Parent organization ID */
  orgId: string;
  /** Repository description */
  description?: string;
  /** URL to the repository */
  url?: string;
}

/**
 * Batch Job status enumeration.
 */
export enum BatchJobStatus {
  PENDING = 'PENDING',
  RUNNING = 'RUNNING',
  COMPLETED = 'COMPLETED',
  FAILED = 'FAILED',
  INTERRUPTED = 'INTERRUPTED',
}

/**
 * Batch Job model.
 */
export interface BatchJob {
  /** Unique identifier */
  id: string;
  /** Target organization or scope */
  target: string;
  /** Current status */
  status: BatchJobStatus;
  /** Creation timestamp */
  createdAt: string;
}

/**
 * Pull Request model.
 */
export interface PullRequest {
  /** Unique identifier */
  id: string;
  /** PR title */
  title: string;
  /** Associated repository ID */
  repoId: string;
  /** Sync status */
  status: 'LOCAL' | 'SYNCED' | 'CONFLICT';
}

/**
 * System Health status.
 */
export interface SystemHealth {
  /** Status of the REST API */
  rest: 'UP' | 'DOWN';
  /** Status of the RPC Server */
  rpc: 'UP' | 'DOWN';
  /** Status of the Agent Daemon */
  agent: 'UP' | 'DOWN';
}
