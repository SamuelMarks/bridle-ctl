#![cfg(not(tarpaulin_include))]
//! Database connection and migration management.

use crate::error::BridleError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// A dynamic connection wrapper supporting both SQLite and PostgreSQL backends.
pub enum DbConnection {
    /// SQLite connection variant.
    Sqlite(SqliteConnection),
    /// PostgreSQL connection variant.
    Pg(PgConnection),
}
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

/// Embedded database migrations.
pub const MIGRATIONS_SQLITE: EmbeddedMigrations = embed_migrations!("migrations");
/// Embedded database migrations for PostgreSQL.
pub const MIGRATIONS_PG: EmbeddedMigrations = embed_migrations!("migrations_pg");

/// Helper to get the database url, with fallbacks.
pub fn database_url() -> String {
    let _ = dotenvy::dotenv();
    std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("POSTGRES_URL"))
        .unwrap_or_else(|_| "bridle.db".to_string())
}

/// Helper function to convert any generic error display into a `BridleError::Database` execution error.
fn db_exec_err<T: std::fmt::Display>(e: T) -> BridleError {
    BridleError::Database(diesel::result::Error::DatabaseError(
        diesel::result::DatabaseErrorKind::UnableToSendCommand,
        Box::new(e.to_string()),
    ))
}

/// Establishes a PostgreSQL database connection and runs pending migrations.
fn establish_pg(database_url: &str) -> Result<DbConnection, BridleError> {
    static MIGRATION_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let mut connection = PgConnection::establish(database_url).map_err(db_exec_err)?;
    let _guard = MIGRATION_MUTEX
        .lock()
        .map_err(|e| BridleError::Generic(format!("Mutex lock failed: {}", e)))?;
    connection
        .run_pending_migrations(MIGRATIONS_PG)
        .map_err(|e| BridleError::Migration(e.to_string()))?;
    Ok(DbConnection::Pg(connection))
}

/// Establishes a database connection and runs pending migrations.
pub fn establish_connection_and_run_migrations(
    database_url: &str,
) -> Result<DbConnection, BridleError> {
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        establish_pg(database_url)
    } else {
        static SQLITE_MIGRATION_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let mut connection = SqliteConnection::establish(database_url).map_err(db_exec_err)?;
        let _guard = SQLITE_MIGRATION_MUTEX
            .lock()
            .map_err(|e| BridleError::Generic(format!("Mutex lock failed: {}", e)))?;
        connection
            .run_pending_migrations(MIGRATIONS_SQLITE)
            .map_err(|e| BridleError::Migration(e.to_string()))?;
        Ok(DbConnection::Sqlite(connection))
    }
}

/// Inserts a new user into the database.
pub fn insert_user(
    conn: &mut DbConnection,
    new_user: &crate::models::User,
) -> Result<(), BridleError> {
    use crate::schema::users::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(users)
                .values(new_user)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(users)
                .values(new_user)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a user from the database by ID.
pub fn get_user(conn: &mut DbConnection, user_id: i32) -> Result<crate::models::User, BridleError> {
    use crate::schema::users::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = users
                .filter(id.eq(user_id))
                .first::<crate::models::User>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = users
                .filter(id.eq(user_id))
                .first::<crate::models::User>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new organisation into the database.
pub fn insert_organisation(
    conn: &mut DbConnection,
    new_org: &crate::models::Organisation,
) -> Result<(), BridleError> {
    use crate::schema::organisations::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(organisations)
                .values(new_org)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(organisations)
                .values(new_org)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves an organisation from the database by ID.
pub fn get_organisation(
    conn: &mut DbConnection,
    org_id: i32,
) -> Result<crate::models::Organisation, BridleError> {
    use crate::schema::organisations::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = organisations
                .filter(id.eq(org_id))
                .first::<crate::models::Organisation>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = organisations
                .filter(id.eq(org_id))
                .first::<crate::models::Organisation>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new repository into the database.
pub fn insert_repository(
    conn: &mut DbConnection,
    new_repo: &crate::models::Repository,
) -> Result<(), BridleError> {
    use crate::schema::repositories::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(repositories)
                .values(new_repo)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(repositories)
                .values(new_repo)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a repository from the database by ID.
pub fn get_repository(
    conn: &mut DbConnection,
    repo_id: i32,
) -> Result<crate::models::Repository, BridleError> {
    use crate::schema::repositories::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = repositories
                .filter(id.eq(repo_id))
                .first::<crate::models::Repository>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = repositories
                .filter(id.eq(repo_id))
                .first::<crate::models::Repository>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Team into the database.
pub fn insert_team(
    conn: &mut DbConnection,
    new_item: &crate::models::Team,
) -> Result<(), BridleError> {
    use crate::schema::teams::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(teams)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(teams)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Team from the database by ID.
pub fn get_team(conn: &mut DbConnection, item_id: i32) -> Result<crate::models::Team, BridleError> {
    use crate::schema::teams::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = teams
                .filter(id.eq(item_id))
                .first::<crate::models::Team>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = teams
                .filter(id.eq(item_id))
                .first::<crate::models::Team>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Branch into the database.
pub fn insert_branch(
    conn: &mut DbConnection,
    new_item: &crate::models::Branch,
) -> Result<(), BridleError> {
    use crate::schema::branches::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(branches)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(branches)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Branch from the database by ID.
pub fn get_branch(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Branch, BridleError> {
    use crate::schema::branches::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = branches
                .filter(id.eq(item_id))
                .first::<crate::models::Branch>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = branches
                .filter(id.eq(item_id))
                .first::<crate::models::Branch>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new BranchProtectionRule into the database.
pub fn insert_branch_protection_rule(
    conn: &mut DbConnection,
    new_item: &crate::models::BranchProtectionRule,
) -> Result<(), BridleError> {
    use crate::schema::branch_protection_rules::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(branch_protection_rules)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(branch_protection_rules)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a BranchProtectionRule from the database by ID.
pub fn get_branch_protection_rule(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::BranchProtectionRule, BridleError> {
    use crate::schema::branch_protection_rules::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = branch_protection_rules
                .filter(id.eq(item_id))
                .first::<crate::models::BranchProtectionRule>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = branch_protection_rules
                .filter(id.eq(item_id))
                .first::<crate::models::BranchProtectionRule>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Key into the database.
pub fn insert_key(
    conn: &mut DbConnection,
    new_item: &crate::models::Key,
) -> Result<(), BridleError> {
    use crate::schema::keys::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(keys)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(keys)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Key from the database by ID.
pub fn get_key(conn: &mut DbConnection, item_id: i32) -> Result<crate::models::Key, BridleError> {
    use crate::schema::keys::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = keys
                .filter(id.eq(item_id))
                .first::<crate::models::Key>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = keys
                .filter(id.eq(item_id))
                .first::<crate::models::Key>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Follow into the database.
pub fn insert_follow(
    conn: &mut DbConnection,
    new_item: &crate::models::Follow,
) -> Result<(), BridleError> {
    use crate::schema::follows::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(follows)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(follows)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Follow from the database by ID.
pub fn get_follow(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Follow, BridleError> {
    use crate::schema::follows::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = follows
                .filter(id.eq(item_id))
                .first::<crate::models::Follow>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = follows
                .filter(id.eq(item_id))
                .first::<crate::models::Follow>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Star into the database.
pub fn insert_star(
    conn: &mut DbConnection,
    new_item: &crate::models::Star,
) -> Result<(), BridleError> {
    use crate::schema::stars::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(stars)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(stars)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Star from the database by ID.
pub fn get_star(conn: &mut DbConnection, item_id: i32) -> Result<crate::models::Star, BridleError> {
    use crate::schema::stars::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = stars
                .filter(id.eq(item_id))
                .first::<crate::models::Star>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = stars
                .filter(id.eq(item_id))
                .first::<crate::models::Star>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new OrgMembership into the database.
pub fn insert_org_membership(
    conn: &mut DbConnection,
    new_item: &crate::models::OrgMembership,
) -> Result<(), BridleError> {
    use crate::schema::org_memberships::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(org_memberships)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(org_memberships)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a OrgMembership from the database by ID.
pub fn get_org_membership(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::OrgMembership, BridleError> {
    use crate::schema::org_memberships::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = org_memberships
                .filter(id.eq(item_id))
                .first::<crate::models::OrgMembership>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = org_memberships
                .filter(id.eq(item_id))
                .first::<crate::models::OrgMembership>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new RepoCollaborator into the database.
pub fn insert_repo_collaborator(
    conn: &mut DbConnection,
    new_item: &crate::models::RepoCollaborator,
) -> Result<(), BridleError> {
    use crate::schema::repo_collaborators::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(repo_collaborators)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(repo_collaborators)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a RepoCollaborator from the database by ID.
pub fn get_repo_collaborator(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::RepoCollaborator, BridleError> {
    use crate::schema::repo_collaborators::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = repo_collaborators
                .filter(id.eq(item_id))
                .first::<crate::models::RepoCollaborator>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = repo_collaborators
                .filter(id.eq(item_id))
                .first::<crate::models::RepoCollaborator>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Milestone into the database.
pub fn insert_milestone(
    conn: &mut DbConnection,
    new_item: &crate::models::Milestone,
) -> Result<(), BridleError> {
    use crate::schema::milestones::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(milestones)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(milestones)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Milestone from the database by ID.
pub fn get_milestone(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Milestone, BridleError> {
    use crate::schema::milestones::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = milestones
                .filter(id.eq(item_id))
                .first::<crate::models::Milestone>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = milestones
                .filter(id.eq(item_id))
                .first::<crate::models::Milestone>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Label into the database.
pub fn insert_label(
    conn: &mut DbConnection,
    new_item: &crate::models::Label,
) -> Result<(), BridleError> {
    use crate::schema::labels::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(labels)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(labels)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Label from the database by ID.
pub fn get_label(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Label, BridleError> {
    use crate::schema::labels::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = labels
                .filter(id.eq(item_id))
                .first::<crate::models::Label>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = labels
                .filter(id.eq(item_id))
                .first::<crate::models::Label>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Issue into the database.
pub fn insert_issue(
    conn: &mut DbConnection,
    new_item: &crate::models::Issue,
) -> Result<(), BridleError> {
    use crate::schema::issues::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(issues)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(issues)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Issue from the database by ID.
pub fn get_issue(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Issue, BridleError> {
    use crate::schema::issues::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = issues
                .filter(id.eq(item_id))
                .first::<crate::models::Issue>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = issues
                .filter(id.eq(item_id))
                .first::<crate::models::Issue>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new IssueLabel into the database.
pub fn insert_issue_label(
    conn: &mut DbConnection,
    new_item: &crate::models::IssueLabel,
) -> Result<(), BridleError> {
    use crate::schema::issue_labels::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(issue_labels)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(issue_labels)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a IssueLabel from the database by ID.
pub fn get_issue_label(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::IssueLabel, BridleError> {
    use crate::schema::issue_labels::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = issue_labels
                .filter(id.eq(item_id))
                .first::<crate::models::IssueLabel>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = issue_labels
                .filter(id.eq(item_id))
                .first::<crate::models::IssueLabel>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new PullRequest into the database.
pub fn insert_pull_request(
    conn: &mut DbConnection,
    new_item: &crate::models::PullRequest,
) -> Result<(), BridleError> {
    use crate::schema::pull_requests::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(pull_requests)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(pull_requests)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a PullRequest from the database by ID.
pub fn get_pull_request(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::PullRequest, BridleError> {
    use crate::schema::pull_requests::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = pull_requests
                .filter(id.eq(item_id))
                .first::<crate::models::PullRequest>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = pull_requests
                .filter(id.eq(item_id))
                .first::<crate::models::PullRequest>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new PullRequestReview into the database.
pub fn insert_pull_request_review(
    conn: &mut DbConnection,
    new_item: &crate::models::PullRequestReview,
) -> Result<(), BridleError> {
    use crate::schema::pull_request_reviews::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(pull_request_reviews)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(pull_request_reviews)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a PullRequestReview from the database by ID.
pub fn get_pull_request_review(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::PullRequestReview, BridleError> {
    use crate::schema::pull_request_reviews::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = pull_request_reviews
                .filter(id.eq(item_id))
                .first::<crate::models::PullRequestReview>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = pull_request_reviews
                .filter(id.eq(item_id))
                .first::<crate::models::PullRequestReview>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Release into the database.
pub fn insert_release(
    conn: &mut DbConnection,
    new_item: &crate::models::Release,
) -> Result<(), BridleError> {
    use crate::schema::releases::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(releases)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(releases)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Release from the database by ID.
pub fn get_release(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Release, BridleError> {
    use crate::schema::releases::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = releases
                .filter(id.eq(item_id))
                .first::<crate::models::Release>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = releases
                .filter(id.eq(item_id))
                .first::<crate::models::Release>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Webhook into the database.
pub fn insert_webhook(
    conn: &mut DbConnection,
    new_item: &crate::models::Webhook,
) -> Result<(), BridleError> {
    use crate::schema::webhooks::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(webhooks)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(webhooks)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Webhook from the database by ID.
pub fn get_webhook(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Webhook, BridleError> {
    use crate::schema::webhooks::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = webhooks
                .filter(id.eq(item_id))
                .first::<crate::models::Webhook>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = webhooks
                .filter(id.eq(item_id))
                .first::<crate::models::Webhook>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Commit into the database.
pub fn insert_commit(
    conn: &mut DbConnection,
    new_item: &crate::models::Commit,
) -> Result<(), BridleError> {
    use crate::schema::commits::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(commits)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(commits)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Commit from the database by ID.
pub fn get_commit(
    conn: &mut DbConnection,
    item_id: i32,
) -> Result<crate::models::Commit, BridleError> {
    use crate::schema::commits::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = commits
                .filter(id.eq(item_id))
                .first::<crate::models::Commit>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = commits
                .filter(id.eq(item_id))
                .first::<crate::models::Commit>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Tree into the database.
pub fn insert_tree(
    conn: &mut DbConnection,
    new_item: &crate::models::Tree,
) -> Result<(), BridleError> {
    use crate::schema::trees::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(trees)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(trees)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Tree from the database by ID.
pub fn get_tree(conn: &mut DbConnection, item_id: i32) -> Result<crate::models::Tree, BridleError> {
    use crate::schema::trees::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = trees
                .filter(id.eq(item_id))
                .first::<crate::models::Tree>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = trees
                .filter(id.eq(item_id))
                .first::<crate::models::Tree>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

/// Inserts a new Blob into the database.
pub fn insert_blob(
    conn: &mut DbConnection,
    new_item: &crate::models::Blob,
) -> Result<(), BridleError> {
    use crate::schema::blobs::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            diesel::insert_into(blobs)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
        crate::db::DbConnection::Pg(c) => {
            diesel::insert_into(blobs)
                .values(new_item)
                .execute(c)
                .map_err(db_exec_err)?;
            Ok(())
        }
    }
}

/// Retrieves a Blob from the database by ID.
pub fn get_blob(conn: &mut DbConnection, item_id: i32) -> Result<crate::models::Blob, BridleError> {
    use crate::schema::blobs::dsl::*;
    match conn {
        crate::db::DbConnection::Sqlite(c) => {
            let fetched = blobs
                .filter(id.eq(item_id))
                .first::<crate::models::Blob>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
        crate::db::DbConnection::Pg(c) => {
            let fetched = blobs
                .filter(id.eq(item_id))
                .first::<crate::models::Blob>(c)
                .map_err(db_exec_err)?;
            Ok(fetched)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_establish_connection_and_run_migrations() {
        let conn = establish_connection_and_run_migrations(":memory:");
        assert!(conn.is_ok());
    }

    #[test]
    fn test_failed_connection() {
        let conn =
            establish_connection_and_run_migrations("///invalid/path/that/does/not/exist.db");
        assert!(conn.is_err());
    }

    #[test]
    fn test_insert_and_get_user() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        let now = chrono::Utc::now().naive_utc();
        let new_user = crate::models::User {
            id: 1,
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };

        insert_user(&mut conn, &new_user)?;
        let duplicate = insert_user(&mut conn, &new_user);
        assert!(duplicate.is_err());

        let fetched = get_user(&mut conn, 1)?;
        assert_eq!(fetched.username, "test");

        let missing = get_user(&mut conn, 999);
        assert!(missing.is_err());

        Ok(())
    }

    #[test]
    fn test_insert_and_get_organisation() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        let now = chrono::Utc::now().naive_utc();
        let new_org = crate::models::Organisation {
            id: 1,
            name: "testorg".to_string(),
            description: None,
            verified_domain: None,
            billing_plan: "free".to_string(),
            created_at: now,
            updated_at: now,
        };

        insert_organisation(&mut conn, &new_org)?;
        let duplicate = insert_organisation(&mut conn, &new_org);
        assert!(duplicate.is_err());

        let fetched = get_organisation(&mut conn, 1)?;
        assert_eq!(fetched.name, "testorg");

        let missing = get_organisation(&mut conn, 999);
        assert!(missing.is_err());

        Ok(())
    }

    #[test]
    fn test_insert_and_get_repository() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;

        let now = chrono::Utc::now().naive_utc();
        let new_repo = crate::models::Repository {
            id: 1,
            owner_id: 1,
            owner_type: "user".to_string(),
            name: "testrepo".to_string(),
            description: None,
            is_private: false,
            is_fork: false,
            archived: false,
            allow_merge_commit: true,
            allow_squash_merge: true,
            allow_rebase_merge: true,
            created_at: now,
            updated_at: now,
        };

        insert_repository(&mut conn, &new_repo)?;
        let duplicate = insert_repository(&mut conn, &new_repo);
        assert!(duplicate.is_err());

        let fetched = get_repository(&mut conn, 1)?;
        assert_eq!(fetched.name, "testrepo");

        let missing = get_repository(&mut conn, 999);
        assert!(missing.is_err());

        Ok(())
    }

    #[test]
    fn test_insert_and_get_team() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Team {
            id: 1,
            org_id: 1,
            parent_id: None,
            name: "devs".into(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        insert_team(&mut conn, &new_item)?;
        let duplicate = insert_team(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_team(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_team(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_branch() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Branch {
            id: 1,
            repo_id: 1,
            name: "main".into(),
            head_sha: "abcdef".into(),
            is_protected: true,
            created_at: now,
            updated_at: now,
        };
        insert_branch(&mut conn, &new_item)?;
        let duplicate = insert_branch(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_branch(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_branch(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_branch_protection_rule() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::BranchProtectionRule {
            id: 1,
            branch_id: 1,
            required_pr_reviews: 1,
            require_code_owner_reviews: true,
            required_status_checks: None,
            require_signed_commits: true,
            enforce_admins: true,
            created_at: now,
            updated_at: now,
        };
        insert_branch_protection_rule(&mut conn, &new_item)?;
        let duplicate = insert_branch_protection_rule(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_branch_protection_rule(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_branch_protection_rule(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_key() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        static COUNTER: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1000);
        let unique_id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let dummy_user = crate::models::User {
            id: unique_id,
            username: format!("test_user_key_{}", unique_id),
            email: format!("key{}@test.com", unique_id),
            password_hash: "hash".into(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        insert_user(&mut conn, &dummy_user)?;

        let new_item = crate::models::Key {
            id: unique_id,
            user_id: unique_id,
            key_type: "ssh".into(),
            title: "my key".into(),
            key_data: "ssh-rsa AAAAB3Nza...".into(),
            fingerprint: format!("SHA256:abcd{}", unique_id),
            last_used_at: None,
            created_at: now,
            updated_at: now,
        };
        insert_key(&mut conn, &new_item)?;
        let duplicate = insert_key(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_key(&mut conn, unique_id)?;
        assert_eq!(fetched.id, unique_id);
        let missing = get_key(&mut conn, 99999);
        assert!(missing.is_err());

        // Test Pg branch
        let url = database_url();
        if url.starts_with("postgres") {
            let mut pg_conn = establish_connection_and_run_migrations(&url)?;
            let pg_id: i32 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() as i32;

            let dummy_user_pg = crate::models::User {
                id: pg_id,
                username: format!("test_user_key_pg_{}", pg_id),
                email: format!("key_pg{}@test.com", pg_id),
                password_hash: "hash".into(),
                avatar_url: None,
                bio: None,
                status: None,
                created_at: now,
                updated_at: now,
            };
            insert_user(&mut pg_conn, &dummy_user_pg)?;

            let pg_item = crate::models::Key {
                id: pg_id,
                user_id: pg_id,
                key_type: "ssh".into(),
                title: "my key pg".into(),
                key_data: "ssh-rsa PG...".into(),
                fingerprint: format!("SHA256:pg{}", pg_id),
                last_used_at: None,
                created_at: now,
                updated_at: now,
            };
            insert_key(&mut pg_conn, &pg_item)?;
            let pg_fetched = get_key(&mut pg_conn, pg_id)?;
            assert_eq!(pg_fetched.id, pg_id);
        }

        Ok(())
    }

    #[test]
    fn test_insert_and_get_follow() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Follow {
            id: 1,
            follower_id: 1,
            following_id: 2,
            created_at: now,
        };
        insert_follow(&mut conn, &new_item)?;
        let duplicate = insert_follow(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_follow(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_follow(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_star() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Star {
            id: 1,
            user_id: 1,
            repo_id: 2,
            created_at: now,
        };
        insert_star(&mut conn, &new_item)?;
        let duplicate = insert_star(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_star(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_star(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_org_membership() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::OrgMembership {
            id: 1,
            org_id: 1,
            user_id: 2,
            role: "owner".into(),
            created_at: now,
            updated_at: now,
        };
        insert_org_membership(&mut conn, &new_item)?;
        let duplicate = insert_org_membership(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_org_membership(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_org_membership(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_repo_collaborator() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::RepoCollaborator {
            id: 1,
            repo_id: 1,
            user_id: 2,
            permission_level: "write".into(),
            created_at: now,
            updated_at: now,
        };
        insert_repo_collaborator(&mut conn, &new_item)?;
        let duplicate = insert_repo_collaborator(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_repo_collaborator(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_repo_collaborator(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_milestone() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Milestone {
            id: 1,
            repo_id: 1,
            title: "v1".into(),
            description: None,
            state: "open".into(),
            due_on: None,
            created_at: now,
            updated_at: now,
        };
        insert_milestone(&mut conn, &new_item)?;
        let duplicate = insert_milestone(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_milestone(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_milestone(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_label() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Label {
            id: 1,
            repo_id: 1,
            name: "bug".into(),
            color: "ff0000".into(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        insert_label(&mut conn, &new_item)?;
        let duplicate = insert_label(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_label(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_label(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_issue() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "bug".into(),
            body: None,
            state: "open".into(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            created_at: now,
            updated_at: now,
        };
        insert_issue(&mut conn, &new_item)?;
        let duplicate = insert_issue(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_issue(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_issue(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_issue_label() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let _now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::IssueLabel {
            id: 1,
            issue_id: 1,
            label_id: 2,
        };
        insert_issue_label(&mut conn, &new_item)?;
        let duplicate = insert_issue_label(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_issue_label(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_issue_label(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_pull_request() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "fix".into(),
            body: None,
            state: "open".into(),
            head_branch: "feat".into(),
            base_branch: "main".into(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            is_draft: false,
            created_at: now,
            updated_at: now,
        };
        insert_pull_request(&mut conn, &new_item)?;
        let duplicate = insert_pull_request(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_pull_request(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_pull_request(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_pull_request_review() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::PullRequestReview {
            id: 1,
            pr_id: 1,
            user_id: 2,
            state: "approved".into(),
            body: None,
            created_at: now,
            updated_at: now,
        };
        insert_pull_request_review(&mut conn, &new_item)?;
        let duplicate = insert_pull_request_review(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_pull_request_review(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_pull_request_review(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_release() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1.0".into(),
            target_commitish: "main".into(),
            name: None,
            body: None,
            is_draft: false,
            is_prerelease: false,
            author_id: 1,
            created_at: now,
            published_at: None,
        };
        insert_release(&mut conn, &new_item)?;
        let duplicate = insert_release(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_release(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_release(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_webhook() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Webhook {
            id: 1,
            repo_id: 1,
            url: "http://test".into(),
            content_type: "json".into(),
            secret: None,
            events: "*".into(),
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        insert_webhook(&mut conn, &new_item)?;
        let duplicate = insert_webhook(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_webhook(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_webhook(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_commit() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Commit {
            id: 1,
            repo_id: 1,
            sha: "abcdef".into(),
            tree_sha: "123456".into(),
            parent_shas: "".into(),
            message: "initial".into(),
            author_name: "test".into(),
            author_email: "test@example.com".into(),
            author_date: now,
            committer_name: "test".into(),
            committer_email: "test@example.com".into(),
            committer_date: now,
            created_at: now,
        };
        insert_commit(&mut conn, &new_item)?;
        let duplicate = insert_commit(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_commit(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_commit(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_tree() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Tree {
            id: 1,
            repo_id: 1,
            sha: "123456".into(),
            entries: "[]".into(),
            created_at: now,
        };
        insert_tree(&mut conn, &new_item)?;
        let duplicate = insert_tree(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_tree(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_tree(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }

    #[test]
    fn test_insert_and_get_blob() -> Result<(), BridleError> {
        let mut conn = establish_connection_and_run_migrations(":memory:")?;
        let now = chrono::Utc::now().naive_utc();
        let new_item = crate::models::Blob {
            id: 1,
            repo_id: 1,
            sha: "789012".into(),
            size: 4,
            content: Some(vec![1, 2, 3, 4]),
            created_at: now,
        };
        insert_blob(&mut conn, &new_item)?;
        let duplicate = insert_blob(&mut conn, &new_item);
        assert!(duplicate.is_err());
        let fetched = get_blob(&mut conn, 1)?;
        assert_eq!(fetched.id, 1);
        let missing = get_blob(&mut conn, 9999);
        assert!(missing.is_err());
        Ok(())
    }
}

#[test]
fn test_schema_parity_sqlite_pg_all() -> Result<(), BridleError> {
    let pg_url = "postgres://samuel@localhost/bridle_test".to_string();
    let mut conn = match establish_connection_and_run_migrations(&pg_url) {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to PG: {:?}", e),
    };

    let now = chrono::Utc::now().naive_utc();
    let id_offset: i32 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|e| panic!("must succeed: {:?}", e))
        .subsec_nanos() as i32;

    let user = crate::models::User {
        id: id_offset,
        username: format!("u{}", id_offset),
        email: format!("u{}@ex.com", id_offset),
        password_hash: "h".into(),
        avatar_url: None,
        bio: None,
        status: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_user(&mut conn, &user);
    let _ = get_user(&mut conn, user.id);

    let org = crate::models::Organisation {
        id: id_offset,
        name: format!("o{}", id_offset),
        description: None,
        verified_domain: None,
        billing_plan: "free".into(),
        created_at: now,
        updated_at: now,
    };
    let _ = insert_organisation(&mut conn, &org);
    let _ = get_organisation(&mut conn, org.id);

    let repo = crate::models::Repository {
        id: id_offset,
        owner_id: user.id,
        owner_type: "user".into(),
        name: format!("r{}", id_offset),
        description: None,
        is_private: false,
        is_fork: false,
        archived: false,
        allow_merge_commit: true,
        allow_squash_merge: true,
        allow_rebase_merge: true,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_repository(&mut conn, &repo);
    let _ = get_repository(&mut conn, repo.id);

    let team = crate::models::Team {
        id: id_offset,
        org_id: org.id,
        parent_id: None,
        name: "t".into(),
        description: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_team(&mut conn, &team);
    let _ = get_team(&mut conn, team.id);

    let branch = crate::models::Branch {
        id: id_offset,
        repo_id: repo.id,
        name: "main".into(),
        head_sha: "s".into(),
        is_protected: false,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_branch(&mut conn, &branch);
    let _ = get_branch(&mut conn, branch.id);

    let bpr = crate::models::BranchProtectionRule {
        id: id_offset,
        branch_id: branch.id,
        required_pr_reviews: 1,
        require_code_owner_reviews: false,
        required_status_checks: None,
        require_signed_commits: false,
        enforce_admins: false,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_branch_protection_rule(&mut conn, &bpr);
    let _ = get_branch_protection_rule(&mut conn, bpr.id);

    let key = crate::models::Key {
        id: id_offset,
        user_id: user.id,
        key_type: "ssh".into(),
        title: "k".into(),
        key_data: "d".into(),
        fingerprint: "f".into(),
        last_used_at: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_key(&mut conn, &key);
    let _ = get_key(&mut conn, key.id);

    let user2 = crate::models::User {
        id: id_offset + 1,
        username: format!("u{}", id_offset + 1),
        email: format!("u{}@ex.com", id_offset + 1),
        password_hash: "h".into(),
        avatar_url: None,
        bio: None,
        status: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_user(&mut conn, &user2);

    let follow = crate::models::Follow {
        id: id_offset,
        follower_id: user.id,
        following_id: user2.id,
        created_at: now,
    };
    let _ = insert_follow(&mut conn, &follow);
    let _ = get_follow(&mut conn, follow.id);

    let star = crate::models::Star {
        id: id_offset,
        user_id: user.id,
        repo_id: repo.id,
        created_at: now,
    };
    let _ = insert_star(&mut conn, &star);
    let _ = get_star(&mut conn, star.id);

    let org_mem = crate::models::OrgMembership {
        id: id_offset,
        org_id: org.id,
        user_id: user.id,
        role: "r".into(),
        created_at: now,
        updated_at: now,
    };
    let _ = insert_org_membership(&mut conn, &org_mem);
    let _ = get_org_membership(&mut conn, org_mem.id);

    let repo_col = crate::models::RepoCollaborator {
        id: id_offset,
        repo_id: repo.id,
        user_id: user.id,
        permission_level: "p".into(),
        created_at: now,
        updated_at: now,
    };
    let _ = insert_repo_collaborator(&mut conn, &repo_col);
    let _ = get_repo_collaborator(&mut conn, repo_col.id);

    let ms = crate::models::Milestone {
        id: id_offset,
        repo_id: repo.id,
        title: "m".into(),
        description: None,
        state: "open".into(),
        due_on: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_milestone(&mut conn, &ms);
    let _ = get_milestone(&mut conn, ms.id);

    let lbl = crate::models::Label {
        id: id_offset,
        repo_id: repo.id,
        name: "l".into(),
        color: "c".into(),
        description: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_label(&mut conn, &lbl);
    let _ = get_label(&mut conn, lbl.id);

    let iss = crate::models::Issue {
        id: id_offset,
        repo_id: repo.id,
        number: 1,
        title: "t".into(),
        body: None,
        state: "open".into(),
        author_id: user.id,
        assignee_id: None,
        milestone_id: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_issue(&mut conn, &iss);
    let _ = get_issue(&mut conn, iss.id);

    let iss_lbl = crate::models::IssueLabel {
        id: id_offset,
        issue_id: iss.id,
        label_id: lbl.id,
    };
    let _ = insert_issue_label(&mut conn, &iss_lbl);
    let _ = get_issue_label(&mut conn, iss_lbl.id);

    let pr = crate::models::PullRequest {
        id: id_offset,
        repo_id: repo.id,
        number: 1,
        title: "t".into(),
        body: None,
        state: "open".into(),
        head_branch: "h".into(),
        base_branch: "b".into(),
        author_id: user.id,
        assignee_id: None,
        milestone_id: None,
        is_draft: false,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_pull_request(&mut conn, &pr);
    let _ = get_pull_request(&mut conn, pr.id);

    let rev = crate::models::PullRequestReview {
        id: id_offset,
        pr_id: pr.id,
        user_id: user.id,
        state: "s".into(),
        body: None,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_pull_request_review(&mut conn, &rev);
    let _ = get_pull_request_review(&mut conn, rev.id);

    let rel = crate::models::Release {
        id: id_offset,
        repo_id: repo.id,
        tag_name: "t".into(),
        target_commitish: "t".into(),
        name: None,
        body: None,
        is_draft: false,
        is_prerelease: false,
        author_id: user.id,
        created_at: now,
        published_at: None,
    };
    let _ = insert_release(&mut conn, &rel);
    let _ = get_release(&mut conn, rel.id);

    let webhook = crate::models::Webhook {
        id: id_offset,
        repo_id: repo.id,
        url: "u".into(),
        content_type: "c".into(),
        secret: None,
        events: "e".into(),
        is_active: false,
        created_at: now,
        updated_at: now,
    };
    let _ = insert_webhook(&mut conn, &webhook);
    let _ = get_webhook(&mut conn, webhook.id);

    let commit = crate::models::Commit {
        id: id_offset,
        repo_id: repo.id,
        sha: "s".into(),
        tree_sha: "t".into(),
        parent_shas: "p".into(),
        message: "m".into(),
        author_name: "a".into(),
        author_email: "e".into(),
        author_date: now,
        committer_name: "c".into(),
        committer_email: "e".into(),
        committer_date: now,
        created_at: now,
    };
    let _ = insert_commit(&mut conn, &commit);
    let _ = get_commit(&mut conn, commit.id);

    let tree = crate::models::Tree {
        id: id_offset,
        repo_id: repo.id,
        sha: "s".into(),
        entries: "e".into(),
        created_at: now,
    };
    let _ = insert_tree(&mut conn, &tree);
    let _ = get_tree(&mut conn, tree.id);

    let blob = crate::models::Blob {
        id: id_offset,
        repo_id: repo.id,
        sha: "s".into(),
        size: 1,
        content: None,
        created_at: now,
    };
    let _ = insert_blob(&mut conn, &blob);
    let _ = get_blob(&mut conn, blob.id);

    Ok(())
}
