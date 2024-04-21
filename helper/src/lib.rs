mod difficulty_ranking;

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub use difficulty_ranking::*;

static ALGO_SRC_DIR: OnceLock<PathBuf> = OnceLock::new();

#[macro_export]
/// Convenient macro to retrive certain local paths from the base source.
macro_rules! algo_src {
    () => {
        crate::get_src_dir()
    };
    ($components:literal) => {
        crate::algo_src!().join($components)
    };
}

/// Get the source directory of the base project.
pub(crate) fn get_src_dir() -> &'static Path {
    ALGO_SRC_DIR.get_or_init(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).with_file_name("src"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn algo_src_macro() {
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).with_file_name("src");
        assert_eq!(algo_src!(), &src_dir);
        assert_eq!(algo_src!("lib"), src_dir.join("lib"));
        assert_eq!(
            algo_src!("backtracking/sudoku.rs"),
            src_dir.join("backtracking").join("sudoku.rs")
        );
    }
}
