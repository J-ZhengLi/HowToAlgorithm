//! Rank the algorithms by difficulties, which basically affected by their
//! line of codes (LOC), folders they are in, and an override list (not available yet).

use anyhow::Result;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use crate::algo_src;

pub struct Algorithm {
    name: String,
    catagory: Option<String>,
    path: PathBuf,
    difficulty: Difficulty,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn rank() -> Result<VecDeque<Algorithm>> {
    let ranked = rank_by_line_count()?;
    println!("ranked: {:?}", ranked);
    Ok(VecDeque::new())
}

pub(crate) fn rank_by_line_count() -> Result<Vec<PathBuf>> {
    fn process_source_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in path.read_dir()? {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                process_source_files(&entry_path, files)?;
            } else if !is_inrelevant_file(&entry_path) {
                files.push(entry_path);
            }
        }
        Ok(())
    }

    let mut res = vec![];
    process_source_files(algo_src!(), &mut res)?;
    Ok(res)
}

/// Return the number of LOC (line of codes) of a file.
///
/// This does not include comments, and test code.
pub(crate) fn count_loc(file: &Path) -> Result<usize> {
    Ok(1)
}

fn is_inrelevant_file(path: &Path) -> bool {
    let Some(filename) = path.file_name() else { return false };
    let Some(extension) = path.extension() else { return false };
    filename == "lib.rs" || filename == "mod.rs" || extension == "md"
}
