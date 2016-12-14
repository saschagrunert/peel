//! The logger implementation
use log::{Log, LogRecord, LogLevel, LogMetadata};

use term::stderr;
use term::color::{BRIGHT_BLUE, GREEN, BRIGHT_YELLOW, RED};

use error::{PealResult, ErrorType};

/// The logging structure
pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            self.log_result(record).is_ok();
        }
    }
}

impl Logger {
    fn log_result(&self, record: &LogRecord) -> PealResult<()> {
        // We have to create a new terminal on each log because Send is not fulfilled
        let mut t = match stderr() {
            Some(terminal) => terminal,
            None => bail!(ErrorType::Internal, "Could not create terminal."),
        };
        t.fg(BRIGHT_BLUE)?;
        write!(t, "[peal] ")?;
        match record.level() {
            LogLevel::Info => {
                t.fg(GREEN)?;
                write!(t, "[OKAY] ")?;
                t.reset()?;
                writeln!(t, "{}", record.args())?;
            }
            LogLevel::Warn => {
                t.fg(BRIGHT_YELLOW)?;
                write!(t, "[WARN] ")?;
                t.reset()?;
                writeln!(t, "{}", record.args())?;
            }
            LogLevel::Error => {
                t.fg(RED)?;
                write!(t, "[ERROR] ")?;
                t.reset()?;
                writeln!(t, "{}", record.args())?;
            }
            _ => {
                write!(t, "[{}] ", record.level())?;
                t.reset()?;
                writeln!(t, "{}", record.args())?;
            }
        }
        Ok(())
    }
}
