//! Schema parity test between SQLite and PostgreSQL migrations.
use std::fs;
use std::path::PathBuf;

#[test]
fn test_schema_parity_sqlite_pg() -> Result<(), bridle_sdk::BridleError> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sqlite_migrations_dir = manifest_dir.join("migrations");
    let pg_migrations_dir = manifest_dir.join("migrations_pg");

    let mut sqlite_migrations: Vec<String> = fs::read_dir(&sqlite_migrations_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name().into_string().ok()?)
            } else {
                None
            }
        })
        .collect();

    let mut pg_migrations: Vec<String> = fs::read_dir(&pg_migrations_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name().into_string().ok()?)
            } else {
                None
            }
        })
        .collect();

    sqlite_migrations.sort();
    pg_migrations.sort();

    // Check that lists match exactly
    assert_eq!(
        sqlite_migrations, pg_migrations,
        "Migration directories do not match between SQLite and PostgreSQL"
    );

    // Additionally check that each directory has an up.sql and down.sql
    for migration in sqlite_migrations {
        let sqlite_up = sqlite_migrations_dir.join(&migration).join("up.sql");
        let sqlite_down = sqlite_migrations_dir.join(&migration).join("down.sql");
        let pg_up = pg_migrations_dir.join(&migration).join("up.sql");
        let pg_down = pg_migrations_dir.join(&migration).join("down.sql");

        assert!(
            sqlite_up.exists(),
            "Missing up.sql in SQLite migration: {}",
            migration
        );
        assert!(
            sqlite_down.exists(),
            "Missing down.sql in SQLite migration: {}",
            migration
        );
        assert!(
            pg_up.exists(),
            "Missing up.sql in PostgreSQL migration: {}",
            migration
        );
        assert!(
            pg_down.exists(),
            "Missing down.sql in PostgreSQL migration: {}",
            migration
        );
    }

    Ok(())
}
