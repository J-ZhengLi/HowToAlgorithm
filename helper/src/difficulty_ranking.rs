//! Rank the algorithms by difficulties, which basically affected by their
//! line of codes (LOC), folders they are in, and an override list (not available yet).

use anyhow::Result;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::algo_src;

#[derive(Debug)]
pub struct Algorithm {
    name: String,
    catagory: String,
    path: PathBuf,
    loc: usize,
}

impl Algorithm {
    fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_stem()?;
        let catagory = path.iter().rev().skip(1).next()?;
        // Panic if it cannot be counted.
        let loc = count_loc(path).expect("failed to count LOC");
        Some(Self {
            name: name.to_string_lossy().to_string(),
            catagory: catagory.to_string_lossy().to_string(),
            path: path.to_path_buf(),
            loc,
        })
    }
}

pub fn sort() -> Result<VecDeque<Algorithm>> {
    let ranked = sort_by_line_count()?;
    println!(
        "ranked: {:#?}",
        ranked.iter().map(|a| a.path.as_path()).collect::<Vec<_>>()
    );
    Ok(VecDeque::from(ranked))
}

pub(crate) fn sort_by_catagory() -> Result<Vec<Algorithm>> {
    Ok(vec![])
}

pub(crate) fn sort_by_line_count() -> Result<Vec<Algorithm>> {
    fn process_source_files(path: &Path, files: &mut Vec<Algorithm>) -> Result<()> {
        for entry in path.read_dir()? {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                process_source_files(&entry_path, files)?;
            } else if !is_inrelevant_file(&entry_path) {
                files.push(Algorithm::from_path(&entry_path).expect("unknown file path"));
            }
        }
        Ok(())
    }

    let mut res = vec![];
    process_source_files(algo_src!(), &mut res)?;
    res.sort_by(|a, b| a.loc.cmp(&b.loc));
    Ok(res)
}

/// Return the number of LOC (line of codes) of a file.
///
/// This does not include comments, and test code.
pub(crate) fn count_loc(path: &Path) -> Result<usize> {
    enum State {
        BlockComment,
        TestModule,
        Regular,
    }

    let mut count = 0;
    let mut state = State::Regular;
    let mut block_level = 0;
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        let trimed = line.trim();
        if trimed.is_empty()
            || trimed.starts_with("//")
            || (trimed.starts_with("/*") && trimed.contains("*/"))
        {
            continue;
        } else if trimed.starts_with("/*") {
            state = State::BlockComment;
        } else if trimed.starts_with("*/") {
            state = State::Regular;
        } else if trimed.starts_with("#[cfg(test)]") {
            state = State::TestModule;
        } else if matches!(state, State::TestModule) && trimed.ends_with('{') {
            block_level += 1;
        } else if matches!(state, State::TestModule) && trimed.ends_with('}') {
            block_level -= 1;
            if block_level == 0 {
                state = State::Regular;
            }
        } else if matches!(state, State::Regular) {
            count += 1;
        }
    }
    Ok(count)
}

fn is_inrelevant_file(path: &Path) -> bool {
    let Some(filename) = path.file_name() else {
        return false;
    };
    let Some(extension) = path.extension() else {
        return false;
    };
    filename == "lib.rs"
        || filename == "mod.rs"
        || filename.to_string_lossy().contains("_utils")
        || extension != "rs"
}
