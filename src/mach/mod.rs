pub mod logger;
mod panic;

use logger::Logger;

///
/// State of the Mach Kernel
///
pub struct Mach {
    /// The logger for this instance
    logger: Option<Logger>,
}

impl Mach {
    pub fn new() -> Mach {
        return Mach { logger: None };
    }
}
