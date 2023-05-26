pub const BAU_LOG_PREFIX: &str = "BILL ACCEPTOR";

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    Off = 0,
    Critical,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<u32> for LogLevel {
    fn from(level: u32) -> Self {
        match level {
            0 => Self::Off,
            1 => Self::Critical,
            2 => Self::Error,
            3 => Self::Warn,
            4 => Self::Info,
            5 => Self::Debug,
            6 => Self::Trace,
            _ => Self::Off,
        }
    }
}

impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => log::LevelFilter::Off,
            LogLevel::Trace => log::LevelFilter::Trace,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Critical | LogLevel::Error => log::LevelFilter::Error,
        }
    }
}
