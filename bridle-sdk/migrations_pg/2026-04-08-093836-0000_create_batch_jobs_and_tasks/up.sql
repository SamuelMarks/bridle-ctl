CREATE TABLE batch_jobs (
    id SERIAL PRIMARY KEY,
    pipeline_name VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE TABLE batch_tasks (
    id SERIAL PRIMARY KEY,
    job_id INTEGER NOT NULL REFERENCES batch_jobs(id),
    repo_id INTEGER NOT NULL REFERENCES repositories(id),
    status VARCHAR NOT NULL,
    error_reason TEXT,
    pr_url VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE task_logs (
    id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES batch_tasks(id),
    step_name VARCHAR NOT NULL,
    stdout TEXT,
    stderr TEXT,
    exit_code INTEGER,
    duration_ms INTEGER
);
