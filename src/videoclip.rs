use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use time::{macros::format_description, Date, Time};

use crate::error::AppError;

pub enum ClipProgram {
    Nvidia,
    AMD,
}

#[derive(Debug, PartialEq)]
pub struct VideoClip {
    pub source_name: String,
    pub date: Date,
    pub time: Time,
}

impl VideoClip {
    /// TODO: Add support for custom formats
    pub fn format_name() -> String {
        format!("- ", name="yes")
    }
}

impl FromStr for VideoClip {
    type Err = AppError;

    /// Example from Nvidia: Monster Hunter Rise 2023.06.11 - 00.31.42.02.DVR.mp4
    /// Example from AMD: MONSTER HUNTER RISE_replay_2023.06.12-01.15.mp4
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clip_program: ClipProgram;
        // chose to not use named capture groups because it is incompatible with https://regex101.com
        static RE_NVIDIA: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(.*) ([0-9]{4}.[0-9]{2}.[0-9]{2}) - (.*)\.([0-9]{2})\.DVR\.mp4$").unwrap()
        });
        static RE_AMD: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(.*)_replay_(.*)-([0-9]{2}\.[0-9]{2})\.mp4$").unwrap());
        // TODO: probably should make clip program detection more robust
        if s.contains("_replay_") {
            clip_program = ClipProgram::AMD
        } else if s.contains(".DVR.") {
            clip_program = ClipProgram::Nvidia
        } else {
            return Err(AppError::ParseVideoClipError(format!(
                "video file {} did not match any known clip programs",
                s
            )));
        }

        // Declare variables that are determined by the Clip Program
        let caps: Option<Captures<'_>>;
        let expected_date_fmt: &[time::format_description::FormatItem];
        let expected_time_fmt: &[time::format_description::FormatItem];

        match clip_program {
            ClipProgram::Nvidia => {
                expected_date_fmt = format_description!("[year].[month].[day]");
                expected_time_fmt = format_description!("[hour].[minute].[second]");
                caps = RE_NVIDIA.captures(s);
            }
            ClipProgram::AMD => {
                expected_date_fmt = format_description!("[year].[month].[day]");
                expected_time_fmt = format_description!("[hour].[minute]");
                caps = RE_AMD.captures(s);
            }
        };

        let Some(caps) = caps else {
            return Err(AppError::UnknownClipProgram(s.into()));
        };

        let Some(source_name_regex) = caps.get(1) else {
            return Err(AppError::MissingAttribute("source_name".into(), s.into()));
        };

        let Some(date_regex) = caps.get(2) else {
            return Err(AppError::MissingAttribute("date".into(), s.into()));
        };

        let Some(time_regex) = caps.get(3) else {
            return Err(AppError::MissingAttribute("time".into(), s.into()));
        };

        let Ok(date) = Date::parse(date_regex.as_str(), &expected_date_fmt) else {
            return Err(AppError::ParseVideoClipError(format!(
                "failed to parse date str {} for video file {}",
                date_regex.as_str(),
                s
            )));
        };

        let Ok(time) = Time::parse(time_regex.as_str(), &expected_time_fmt) else {
            return Err(AppError::ParseVideoClipError(format!(
                "failed to parse time str {} for video file {}",
                time_regex.as_str(),
                s
            )));
        };

        Ok(VideoClip {
            source_name: source_name_regex.as_str().to_string(),
            date,
            time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::VideoClip;

    #[test]
    fn check_nvidia_filename_works() {
        let nvidia_file = "Monster Hunter Rise 2023.06.11 - 00.31.42.02.DVR.mp4";
        let n = nvidia_file.parse::<VideoClip>().unwrap();

        // let date_fmt = format_description!("[year]-[month]-[day]");
        // assert_eq!(n.date.format(&format).unwrap(), "2023-06-11");
        assert_eq!(n.date, time::macros::date!(2023 - 06 - 11));
    }

    #[test]

    fn check_amd_filename_works() {
        let amd_file = "MONSTER HUNTER RISE_replay_2023.06.12-01.15.mp4";
        let n = amd_file.parse::<VideoClip>().unwrap();

        assert_eq!(n.date, time::macros::date!(2023 - 06 - 12));
    }
}
