use crate::prelude::*;

use time::macros::format_description;
use tracing::Level;
use tracing_subscriber::{
    fmt::{time::LocalTime, writer::MakeWriterExt, Layer},
    prelude::__tracing_subscriber_SubscriberExt,
};

/// Sets up a global logger for the whole application
pub fn setup_logging(log_level: Level) -> Result<()> {
    // Documentation on implemention a Layer wrapped in an Option
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/

    let stdout = std::io::stdout.with_max_level(log_level);

    let time_format = LocalTime::new(format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]"));

    let subscriber = tracing_subscriber::registry()
        .with(Layer::new().with_writer(stdout).with_timer(time_format));

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global logging instance");

    Ok(())
}
