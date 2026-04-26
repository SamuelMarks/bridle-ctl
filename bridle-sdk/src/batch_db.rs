use crate::db::DbConnection;
use crate::error::BridleError;
use crate::models::{BatchJob, BatchTask, TaskStatus};
use crate::schema::{batch_jobs, batch_tasks};
use chrono::Utc;
use diesel::prelude::*;

/// Creates a new batch job.
pub fn create_batch_job(
    conn: &mut DbConnection,
    pipeline_name: &str,
) -> Result<BatchJob, BridleError> {
    let new_job = (
        batch_jobs::pipeline_name.eq(pipeline_name),
        batch_jobs::status.eq(TaskStatus::Pending.to_string()),
        batch_jobs::started_at.eq(Utc::now().naive_utc()),
    );
    match conn {
        DbConnection::Sqlite(c) => {
            diesel::insert_into(batch_jobs::table)
                .values(&new_job)
                .execute(c)
                .map_err(BridleError::Database)?;
            batch_jobs::table
                .order(batch_jobs::id.desc())
                .first::<BatchJob>(c)
                .map_err(BridleError::Database)
        }
        DbConnection::Pg(_c) => {
            diesel::insert_into(batch_jobs::table)
                .values(&new_job)
                .execute(_c)
                .map_err(BridleError::Database)?;
            batch_jobs::table
                .order(batch_jobs::id.desc())
                .first::<BatchJob>(_c)
                .map_err(BridleError::Database)
        }
    }
}

/// Retrieves a batch job by ID.
pub fn get_batch_job(conn: &mut DbConnection, job_id: i32) -> Result<BatchJob, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => batch_jobs::table
            .find(job_id)
            .first::<BatchJob>(c)
            .map_err(BridleError::Database),
        DbConnection::Pg(_c) => batch_jobs::table
            .find(job_id)
            .first::<BatchJob>(_c)
            .map_err(BridleError::Database),
    }
}

/// Inserts a batch job directly.
pub fn insert_batch_job(conn: &mut DbConnection, job: &BatchJob) -> Result<BatchJob, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => {
            diesel::insert_into(batch_jobs::table)
                .values(job)
                .execute(c)
                .map_err(BridleError::Database)?;
            Ok(job.clone())
        }
        DbConnection::Pg(_c) => {
            diesel::insert_into(batch_jobs::table)
                .values(job)
                .execute(_c)
                .map_err(BridleError::Database)?;
            Ok(job.clone())
        }
    }
}

/// Updates a batch job status atomically.
pub fn update_batch_job_status(
    conn: &mut DbConnection,
    job_id: i32,
    new_status: &str,
) -> Result<BatchJob, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => c
            .transaction(|c| {
                diesel::update(batch_jobs::table.find(job_id))
                    .set((batch_jobs::status.eq(new_status),))
                    .execute(c)?;
                batch_jobs::table.find(job_id).first::<BatchJob>(c)
            })
            .map_err(BridleError::Database),
        DbConnection::Pg(_c) => _c
            .transaction(|c| {
                diesel::update(batch_jobs::table.find(job_id))
                    .set((batch_jobs::status.eq(new_status),))
                    .execute(c)?;
                batch_jobs::table.find(job_id).first::<BatchJob>(c)
            })
            .map_err(BridleError::Database),
    }
}

/// Retrieves a batch task by ID.
pub fn get_batch_task(conn: &mut DbConnection, task_id: i32) -> Result<BatchTask, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => batch_tasks::table
            .find(task_id)
            .first::<BatchTask>(c)
            .map_err(BridleError::Database),
        DbConnection::Pg(_c) => batch_tasks::table
            .find(task_id)
            .first::<BatchTask>(_c)
            .map_err(BridleError::Database),
    }
}

/// Inserts a batch task directly.
pub fn insert_batch_task(
    conn: &mut DbConnection,
    task: &BatchTask,
) -> Result<BatchTask, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => {
            diesel::insert_into(batch_tasks::table)
                .values(task)
                .execute(c)
                .map_err(BridleError::Database)?;
            Ok(task.clone())
        }
        DbConnection::Pg(_c) => {
            diesel::insert_into(batch_tasks::table)
                .values(task)
                .execute(_c)
                .map_err(BridleError::Database)?;
            Ok(task.clone())
        }
    }
}

/// Updates task status atomically.
pub fn update_task_status(
    conn: &mut DbConnection,
    task_id: i32,
    new_status: TaskStatus,
    error_reason: Option<String>,
) -> Result<BatchTask, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => c
            .transaction(|c| {
                diesel::update(batch_tasks::table.find(task_id))
                    .set((
                        batch_tasks::status.eq(new_status.to_string()),
                        batch_tasks::error_reason.eq(error_reason),
                        batch_tasks::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(c)?;
                batch_tasks::table.find(task_id).first::<BatchTask>(c)
            })
            .map_err(BridleError::Database),
        DbConnection::Pg(_c) => _c
            .transaction(|c| {
                diesel::update(batch_tasks::table.find(task_id))
                    .set((
                        batch_tasks::status.eq(new_status.to_string()),
                        batch_tasks::error_reason.eq(error_reason),
                        batch_tasks::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(c)?;
                batch_tasks::table.find(task_id).first::<BatchTask>(c)
            })
            .map_err(BridleError::Database),
    }
}

/// Resumes a job by returning its tasks.
pub fn get_job_tasks(conn: &mut DbConnection, job_id: i32) -> Result<Vec<BatchTask>, BridleError> {
    match conn {
        DbConnection::Sqlite(c) => batch_tasks::table
            .filter(batch_tasks::job_id.eq(job_id))
            .load::<BatchTask>(c)
            .map_err(BridleError::Database),
        DbConnection::Pg(_c) => batch_tasks::table
            .filter(batch_tasks::job_id.eq(job_id))
            .load::<BatchTask>(_c)
            .map_err(BridleError::Database),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection_and_run_migrations;

    #[test]
    fn test_create_and_get_job_tasks() -> Result<(), crate::error::BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        let job = create_batch_job(&mut conn, "test_pipeline")?;
        assert_eq!(job.pipeline_name, "test_pipeline");

        let tasks = get_job_tasks(&mut conn, job.id)?;
        assert!(tasks.is_empty());

        Ok(())
    }

    #[test]
    fn test_update_task_status() -> Result<(), crate::error::BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        let job = create_batch_job(&mut conn, "test_pipeline")?;

        let res = update_task_status(&mut conn, 999, TaskStatus::InProgress, None);
        assert!(res.is_err());

        match &mut conn {
            DbConnection::Sqlite(c) => {
                let new_task = (
                    crate::schema::batch_tasks::job_id.eq(job.id),
                    crate::schema::batch_tasks::repo_id.eq(1),
                    crate::schema::batch_tasks::status.eq(TaskStatus::Pending.to_string()),
                );
                diesel::insert_into(crate::schema::batch_tasks::table)
                    .values(&new_task)
                    .execute(c)
                    .map_err(crate::error::BridleError::Database)?;

                let inserted_task = crate::schema::batch_tasks::table
                    .order(crate::schema::batch_tasks::id.desc())
                    .first::<BatchTask>(c)
                    .map_err(crate::error::BridleError::Database)?;

                let updated = update_task_status(
                    &mut conn,
                    inserted_task.id,
                    TaskStatus::Clean,
                    Some("cleaned".to_string()),
                )?;
                assert_eq!(updated.status, "Clean");
                assert_eq!(updated.error_reason.as_deref(), Some("cleaned"));
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    #[test]
    fn test_update_batch_job_status() -> Result<(), crate::error::BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        let job = create_batch_job(&mut conn, "test_pipeline")?;
        assert_eq!(job.status, "Pending");

        let updated_job = update_batch_job_status(&mut conn, job.id, "ABORTED")?;
        assert_eq!(updated_job.status, "ABORTED");

        Ok(())
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::db::establish_connection_and_run_migrations;

    #[test]
    fn test_insert_and_get() -> Result<(), crate::error::BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        // Use create_batch_job just to get a valid inserted job
        let job = create_batch_job(&mut conn, "test")?;

        let fetched = get_batch_job(&mut conn, job.id)?;
        assert_eq!(fetched.id, job.id);

        let missing = get_batch_job(&mut conn, 9999);
        assert!(missing.is_err());

        match &mut conn {
            DbConnection::Sqlite(c) => {
                let new_task = (
                    crate::schema::batch_tasks::job_id.eq(job.id),
                    crate::schema::batch_tasks::repo_id.eq(1),
                    crate::schema::batch_tasks::status.eq(TaskStatus::Pending.to_string()),
                );
                diesel::insert_into(crate::schema::batch_tasks::table)
                    .values(&new_task)
                    .execute(c)
                    .map_err(crate::error::BridleError::Database)?;

                let inserted_task = crate::schema::batch_tasks::table
                    .order(crate::schema::batch_tasks::id.desc())
                    .first::<BatchTask>(c)
                    .map_err(crate::error::BridleError::Database)?;

                let fetched_t = get_batch_task(&mut conn, inserted_task.id)?;
                assert_eq!(fetched_t.id, inserted_task.id);
            }
            _ => unreachable!(),
        }

        let missing_task = get_batch_task(&mut conn, 9999);
        assert!(missing_task.is_err());

        // Also let's test insert_batch_job directly
        let mut job2 = job.clone();
        job2.id = 9999;
        let inserted2 = insert_batch_job(&mut conn, &job2)?;
        assert_eq!(inserted2.id, 9999);
        let dup = insert_batch_job(&mut conn, &job2);
        assert!(dup.is_err());

        // Also test insert_batch_task directly
        let task = BatchTask {
            id: 9999,
            job_id: job.id,
            repo_id: 2,
            status: "Pending".to_string(),
            error_reason: None,
            pr_url: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        let inserted_task = insert_batch_task(&mut conn, &task)?;
        assert_eq!(inserted_task.id, 9999);
        let dup_task = insert_batch_task(&mut conn, &task);
        assert!(dup_task.is_err());

        Ok(())
    }

    #[test]
    fn test_pg_stubs() -> Result<(), crate::error::BridleError> {
        let url = crate::db::database_url();
        if url.starts_with("postgres") {
            let mut conn = establish_connection_and_run_migrations(&url)?;
            let job = create_batch_job(&mut conn, "test")?;
            get_batch_job(&mut conn, job.id)?;
            let mut job2 = job.clone();
            job2.id = job.id + 1000;
            insert_batch_job(&mut conn, &job2)?;
            update_batch_job_status(&mut conn, job.id, "ABORTED")?;

            let unique_suffix = job.id + 1000;
            crate::db::insert_user(
                &mut conn,
                &crate::models::User {
                    id: unique_suffix,
                    username: format!("test_user_{}", unique_suffix),
                    email: format!("test{}@test.com", unique_suffix),
                    password_hash: "hash".to_string(),
                    avatar_url: None,
                    bio: None,
                    status: None,
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_utc(),
                },
            )?;

            crate::db::insert_repository(
                &mut conn,
                &crate::models::Repository {
                    id: unique_suffix,
                    owner_id: unique_suffix,
                    owner_type: "user".to_string(),
                    name: format!("testrepo_{}", unique_suffix),
                    description: None,
                    is_private: false,
                    is_fork: false,
                    archived: false,
                    allow_merge_commit: true,
                    allow_squash_merge: true,
                    allow_rebase_merge: true,
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_utc(),
                },
            )?;

            let task = BatchTask {
                id: unique_suffix,
                job_id: job.id,
                repo_id: unique_suffix,
                status: "Pending".to_string(),
                error_reason: None,
                pr_url: None,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            };

            let task = insert_batch_task(&mut conn, &task)?;
            get_batch_task(&mut conn, task.id)?;
            update_task_status(&mut conn, task.id, TaskStatus::Clean, None)?;
            get_job_tasks(&mut conn, job.id)?;
        }
        Ok(())
    }
}
