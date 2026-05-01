#![cfg(not(tarpaulin_include))]
//! Provides safe concurrent file modification through per-file locking.

use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use fs4::fs_std::FileExt;

use crate::BridleError;
use crate::path_scope::PathScope;

/// Safely mutates a file by acquiring an exclusive lock.
///
/// The `mutator` closure receives the current contents of the file.
/// If `mutator` returns `Some(new_contents)`, the file is overwritten.
/// If `mutator` returns `None`, the file is left unchanged.
///
/// This function prevents concurrent mutation race conditions by
/// utilizing an OS-level exclusive file lock.
pub fn mutate_file_exclusively<P, F>(
    path: P,
    scope: Option<&PathScope>,
    mutator: F,
) -> Result<bool, BridleError>
where
    P: AsRef<Path>,
    F: FnOnce(&str) -> Option<String>,
{
    let path = path.as_ref();

    if scope.is_some_and(|s| !s.is_allowed(path)) {
        return Err(BridleError::Config(format!(
            "Path scope violation for path: {}",
            path.display()
        )));
    }

    let mut file = OpenOptions::new().read(true).write(true).open(path)?;

    file.lock_exclusive()?;

    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);

    {
        if let Err(e) = read_result {
            file.unlock()?;
            return Err(e.into());
        }
    }

    let modified = match mutator(&contents) {
        Some(new_contents) => {
            let write_result = file
                .set_len(0)
                .and_then(|_| file.seek(SeekFrom::Start(0)))
                .and_then(|_| file.write_all(new_contents.as_bytes()));

            {
                if let Err(e) = write_result {
                    file.unlock()?;
                    return Err(e.into());
                }
            }
            true
        }
        None => false,
    };

    file.unlock()?;

    Ok(modified)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_mutate_file_exclusively_modifies_file() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir().map_err(BridleError::Io)?;
        let path = temp_dir.path().join("test1.txt");

        let mut f = File::create(&path).map_err(BridleError::Io)?;
        f.write_all(b"initial contents").map_err(BridleError::Io)?;
        drop(f);

        let modified = mutate_file_exclusively(&path, None, |contents| {
            assert_eq!(contents, "initial contents");
            Some("new contents".to_string())
        })?;

        assert!(modified);

        let mut new_contents = String::new();
        File::open(&path)
            .map_err(BridleError::Io)?
            .read_to_string(&mut new_contents)
            .map_err(BridleError::Io)?;
        assert_eq!(new_contents, "new contents");

        Ok(())
    }

    #[test]
    fn test_mutate_file_exclusively_no_modification() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir().map_err(BridleError::Io)?;
        let path = temp_dir.path().join("test2.txt");

        let mut f = File::create(&path).map_err(BridleError::Io)?;
        f.write_all(b"initial contents").map_err(BridleError::Io)?;
        drop(f);

        let modified = mutate_file_exclusively(&path, None, |_| None)?;

        assert!(!modified);

        let mut new_contents = String::new();
        File::open(&path)
            .map_err(BridleError::Io)?
            .read_to_string(&mut new_contents)
            .map_err(BridleError::Io)?;
        assert_eq!(new_contents, "initial contents");

        Ok(())
    }

    #[test]
    fn test_mutate_file_exclusively_file_not_found() {
        let result = mutate_file_exclusively("non_existent_file.txt", None, |_| None);
        assert!(result.is_err());
    }

    #[test]
    fn test_mutate_file_exclusively_scope_violation() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir().map_err(BridleError::Io)?;
        let path = temp_dir.path().join("test_scope.txt");
        let mut f = File::create(&path).map_err(BridleError::Io)?;
        f.write_all(b"initial contents").map_err(BridleError::Io)?;
        drop(f);

        let scope = PathScope::new(&[], &["**/test_scope.txt".to_string()])?;
        let err = mutate_file_exclusively(&path, Some(&scope), |_| Some("new".to_string()));
        assert!(matches!(err, Err(BridleError::Config(_))));

        Ok(())
    }
}
