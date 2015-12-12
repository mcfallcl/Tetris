extern crate log;

use log::{LogLevel, LogLevelFilter, LogRecord, LogMetadata, SetLoggerError};

struct WarnLogger;
struct AllLogger;

impl log::Log for WarnLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Warn
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

impl log::Log for AllLogger {
    #[allow(unused_variables)]
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        println!("{} - {}", record.level(), record.args());
    }
}

pub fn init_debug() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(AllLogger)
    })
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Warn);
        Box::new(WarnLogger)
    })
}
