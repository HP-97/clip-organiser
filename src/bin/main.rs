use crate::prelude::*;

use rs_cli_template::{
    cli::parse_args,
    get_all_source_videos, prelude,
    utils::{config::AppConfig, logger},
    videoclip::VideoClip,
};
use std::{process::exit, str::FromStr};
use time::format_description::parse;
use tracing::Level;

fn main() -> Result<()> {
    let m = parse_args();
    let cfg = AppConfig::new(&m)?;

    if cfg.log_level > 0 {
        let log_level = match tracing::Level::from_str(&cfg.log_level.to_string()) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("invalid tracing level = {}", &cfg.log_level);
                exit(1)
            }
        };
        logger::setup_logging(log_level)?;
    }
    tracing::event!(Level::DEBUG, "program START");

    let supported_file_exts: Vec<String> = vec!["mp4".into()];
    let files = get_all_source_videos(&cfg.source_dir, &supported_file_exts).unwrap();
    println!("{:?}", files);

    let res = files
        .iter()
        .filter_map(|v| v.file_name().and_then(|s| s.to_str()))
        .collect::<Vec<_>>();

    println!("{:?}", res);

    let mut parse_errors: Vec<AppError> = Vec::new();
    let video_clips = res
        .iter()
        .map(|v| v.parse::<VideoClip>())
        .filter_map(|r| r.map_err(|e| parse_errors.push(e)).ok())
        .collect::<Vec<VideoClip>>();

    println!("{:?}", video_clips);
    println!("printing parse video errors:");
    for err in parse_errors {
        println!("{}", err);
    }

    Ok(())
}
