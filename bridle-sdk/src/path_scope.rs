//! Provides blast radius scoping for file paths.

use crate::BridleError;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

/// Validates whether a file path is allowed to be modified based on allowed/ignored globs.
#[derive(Debug, Clone)]
pub struct PathScope {
    /// The glob set for allowed paths.
    allowed: GlobSet,
    /// The glob set for ignored paths.
    ignored: GlobSet,
}

impl PathScope {
    /// Creates a new PathScope from string representations of globs.
    pub fn new(allowed_globs: &[String], ignored_globs: &[String]) -> Result<Self, BridleError> {
        let mut allowed_builder = GlobSetBuilder::new();
        for g in allowed_globs {
            let glob = Glob::new(g).map_err(|e| BridleError::Config(e.to_string()))?;
            allowed_builder.add(glob);
        }
        let allowed = allowed_builder
            .build()
            .map_err(|e| BridleError::Config(e.to_string()))?;

        let mut ignored_builder = GlobSetBuilder::new();
        for g in ignored_globs {
            let glob = Glob::new(g).map_err(|e| BridleError::Config(e.to_string()))?;
            ignored_builder.add(glob);
        }
        let ignored = ignored_builder
            .build()
            .map_err(|e| BridleError::Config(e.to_string()))?;

        Ok(Self { allowed, ignored })
    }

    /// Checks if a path is allowed to be modified.
    /// If `allowed_globs` is empty, all paths are allowed unless ignored.
    pub fn is_allowed<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        if self.ignored.is_match(path) {
            return false;
        }

        if self.allowed.is_empty() {
            return true;
        }

        self.allowed.is_match(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_scope_empty_allows_all() -> Result<(), BridleError> {
        let scope = PathScope::new(&[], &[])?;
        assert!(scope.is_allowed("src/main.rs"));
        assert!(scope.is_allowed(".github/workflows/ci.yml"));
        Ok(())
    }

    #[test]
    fn test_path_scope_ignored() -> Result<(), BridleError> {
        let scope = PathScope::new(&[], &[".github/**".to_string(), "Cargo.toml".to_string()])?;
        assert!(scope.is_allowed("src/main.rs"));
        assert!(!scope.is_allowed(".github/workflows/ci.yml"));
        assert!(!scope.is_allowed("Cargo.toml"));
        Ok(())
    }

    #[test]
    fn test_path_scope_allowed() -> Result<(), BridleError> {
        let scope = PathScope::new(&["src/**/*.rs".to_string()], &[])?;
        assert!(scope.is_allowed("src/main.rs"));
        assert!(scope.is_allowed("src/utils/helper.rs"));
        assert!(!scope.is_allowed("tests/integration.rs"));
        assert!(!scope.is_allowed("Cargo.toml"));
        Ok(())
    }

    #[test]
    fn test_path_scope_allowed_and_ignored() -> Result<(), BridleError> {
        let scope = PathScope::new(
            &["src/**/*.rs".to_string()],
            &["src/generated/*.rs".to_string()],
        )?;
        assert!(scope.is_allowed("src/main.rs"));
        assert!(!scope.is_allowed("src/generated/api.rs"));
        assert!(!scope.is_allowed("Cargo.toml"));
        Ok(())
    }

    #[test]
    fn test_invalid_glob() {
        let err = PathScope::new(&["[invalid".to_string()], &[]);
        assert!(err.is_err());
    }
}
