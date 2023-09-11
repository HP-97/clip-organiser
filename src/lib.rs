use std::{fs, path::PathBuf};

use crate::prelude::*;
use tracing::Level;
use walkdir::{DirEntry, WalkDir};

pub mod cli;
pub mod error;
pub mod prelude;
pub mod utils;
pub mod videoclip;

/// Returns true if the file has a supported file extension. False otherwise
fn is_supported_file_ext(entry: &DirEntry, supported_file_exts: &Vec<String>) -> bool {
    if let Some(file_name) = entry.file_name().to_str() {
        if let Some(file_ext) = file_name.split(".").last() {
            if supported_file_exts.contains(&file_ext.to_owned()) {
                return true;
            } else {
                tracing::event!(
                    Level::TRACE,
                    "did not match {:?} {:?}",
                    entry,
                    supported_file_exts
                )
            }
        }
    }

    false
}

/// Return a vector that contains all of the found videos. Used to retrieve all videos from the source directory
pub fn get_all_source_videos(
    source_dir: &PathBuf,
    supported_file_exts: &Vec<String>,
) -> Result<Vec<PathBuf>> {
    let mut video_files: Vec<PathBuf> = Vec::new();
    let absolute_path: PathBuf;

    // convert path to absolute path
    match std::fs::canonicalize(source_dir) {
        Ok(v) => absolute_path = v,
        Err(e) => {
            tracing::event!(Level::ERROR, "failed to convert path to absolute path: {e}");
            return Err(AppError::Generic(e.to_string()));
        }
    }

    for entry in WalkDir::new(absolute_path).into_iter() {
        match entry {
            Ok(v) => {
                if v.file_type().is_file() {
                    if is_supported_file_ext(&v, supported_file_exts) {
                        video_files.push(v.into_path());
                    }
                }
            }
            Err(e) => tracing::event!(Level::DEBUG, "failed to process {} - moving on", e),
        }
    }

    Ok(video_files)
}
