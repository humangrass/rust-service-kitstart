use log::LevelFilter;
use cli::LogLevel;

pub fn new_envlogger(log_level: LogLevel) {
    let log_level_filter = match log_level {
        LogLevel::Trace => LevelFilter::Trace,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Error => LevelFilter::Error,
    };

    env_logger::Builder::from_default_env()
        .filter_level(log_level_filter)
        .init();
}
