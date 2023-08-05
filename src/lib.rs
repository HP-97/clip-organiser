use std::path::PathBuf;

use crate::prelude::*;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use walkdir::{DirEntry, WalkDir};

pub mod cli;
pub mod error;
pub mod prelude;
pub mod utils;

fn _get_program_path(binary_name: &str) -> Option<PathBuf> {
    match which::which(binary_name) {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}

/// Returns true if the file has a supported file extension. False otherwise
fn is_supported_file_ext(entry: &DirEntry, supported_file_exts: &Vec<String>) -> bool {
    if let Some(file_name) = entry.file_name().to_str() {
        if let Some(file_ext) = file_name.split(".").last() {
            if supported_file_exts.contains(&file_ext.to_owned()) {
                return true;
            }
        }
    }

    false
}

/// Return a vector that contains all of the found videos. Used to retrieve all videos from the source directory
pub fn get_all_source_videos(
    source_dir: &PathBuf,
    supported_file_exts: &Vec<String>,
) -> Vec<PathBuf> {
    let video_files: Vec<PathBuf> = Vec::new();

    // let video_files =
    println!("{}", source_dir.display());
    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_entry(|v| is_supported_file_ext(v, supported_file_exts))
    {
        println!("{}", entry.unwrap().path().display());
    }

    video_files
}
