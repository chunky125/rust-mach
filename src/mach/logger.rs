use heapless::{String, Vec};

///
/// Different log levels
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LogLevel {
    LogLevelInfo = 1,
    LogLevelDebug = 2,
    LogLevelWarning = 3,
    LogLevelError = 4,
    LogLevelPanic = 5,
}

///
/// Define a macro to help us log
///
macro_rules! log {
    ($log_level:expr , $message:expr) => {{}};
}

///
/// Trait for a logging device, could be serial
/// console, or whatever
///
pub trait LoggingDevice {
    fn write(&mut self, buf: &[u8]) -> ();
}

///
/// Represents our current logger instance
///
pub struct Logger {
    log_level: LogLevel,
    buffer: Vec<String<256>, 20>,
}

impl Logger {
    pub fn create(new_log_level: LogLevel) -> Logger {
        Logger {
            log_level: new_log_level,
            buffer: Vec::<String<256>, 20>::new(),
        }
    }

    pub fn log_to_queue(&mut self, msg_log_level: LogLevel, msg_text: &str) -> () {
        if msg_log_level >= self.log_level {}
    }

    pub fn log_to_device(
        &mut self,
        log_device: Option<&impl LoggingDevice>,
        msg_log_level: LogLevel,
        msg_text: &str,
    ) -> () {
        if msg_log_level >= self.log_level {
            if log_device.is_some() {}
        }
    }
}
