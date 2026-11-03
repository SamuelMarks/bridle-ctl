//! Provides safe concurrent file modification through per-file locking.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::path::Path;

use fs4::fs_std::FileExt;

use crate::BridleError;
use crate::path_scope::PathScope;

/// An RAII guard that holds an exclusive OS-level file lock and unlocks on drop.
struct ExclusiveFileLock {
    /// The locked file handle.
    file: File,
}

impl ExclusiveFileLock {
    /// Acquires an exclusive lock on the given `File`.
    fn acquire(file: File) -> Result<Self, BridleError> {
        file.lock_exclusive()?;
        Ok(Self { file })
    }
}

impl Deref for ExclusiveFileLock {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for ExclusiveFileLock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

impl Drop for ExclusiveFileLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

/// Safely mutates a file by acquiring an exclusive lock via an RAII guard.
///
/// The `mutator` closure receives the current contents of the file.
/// If `mutator` returns `Some(new_contents)`, the file is overwritten.
/// If `mutator` returns `None`, the file is left unchanged.
///
/// This function prevents concurrent mutation race conditions by
/// utilizing an OS-level exclusive file lock that is automatically released on drop.
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

    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut locked_file = ExclusiveFileLock::acquire(file)?;

    let mut contents = String::new();
    locked_file.read_to_string(&mut contents)?;

    let modified = match mutator(&contents) {
        Some(new_contents) => {
            locked_file.set_len(0)?;
            locked_file.seek(SeekFrom::Start(0))?;
            locked_file.write_all(new_contents.as_bytes())?;
            true
        }
        None => false,
    };

    Ok(modified)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_mutate_file_exclusively_modifies_file() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().join("test1.txt");

        let mut f = File::create(&path)?;
        f.write_all(b"initial contents")?;
        drop(f);

        let scope = PathScope::new(&["**/test1.txt".to_string()], &[])?;
        let modified = mutate_file_exclusively(&path, Some(&scope), |contents| {
            assert_eq!(contents, "initial contents");
            Some("new contents".to_string())
        })?;

        assert!(modified);

        let mut new_contents = String::new();
        File::open(&path)?.read_to_string(&mut new_contents)?;
        assert_eq!(new_contents, "new contents");

        Ok(())
    }

    #[test]
    fn test_mutate_file_exclusively_no_modification() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().join("test2.txt");

        let mut f = File::create(&path)?;
        f.write_all(b"initial contents")?;
        drop(f);

        let modified = mutate_file_exclusively(&path, None, |_| None)?;

        assert!(!modified);

        let mut new_contents = String::new();
        File::open(&path)?.read_to_string(&mut new_contents)?;
        assert_eq!(new_contents, "initial contents");

        Ok(())
    }

    #[test]
    fn test_mutate_file_exclusively_file_not_found() {
        let result = mutate_file_exclusively("non_existent_file.txt", None, |_| None);
        assert!(matches!(result, Err(BridleError::Io(_))));
    }

    #[test]
    fn test_mutate_file_exclusively_invalid_utf8_unlocks_cleanly() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().join("invalid_utf8.bin");
        std::fs::write(&path, [0xff, 0xfe, 0xfd])?;

        let result = mutate_file_exclusively(&path, None, |_| Some("updated".to_string()));
        assert!(matches!(result, Err(BridleError::Io(_))));

        // Ensure lock was released by ExclusiveFileLock Drop so we can re-acquire it
        let f = OpenOptions::new().read(true).write(true).open(&path)?;
        let lock = ExclusiveFileLock::acquire(f)?;
        assert!(lock.metadata().is_ok());

        Ok(())
    }

    #[test]
    fn test_mutate_file_exclusively_scope_violation() -> Result<(), BridleError> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().join("test_scope.txt");
        let mut f = File::create(&path)?;
        f.write_all(b"initial contents")?;
        drop(f);

        let scope = PathScope::new(&[], &["**/test_scope.txt".to_string()])?;
        let err = mutate_file_exclusively(&path, Some(&scope), |_| Some("new".to_string()));
        assert!(matches!(err, Err(BridleError::Config(_))));

        Ok(())
    }
}
