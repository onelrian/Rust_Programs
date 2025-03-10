// main.rs

mod logger;
mod system;

use logger::{FileLogger, ConsoleLogger};
use system::System;

fn main() {
    // Use FileLogger
    let file_logger = FileLogger;
    let system_with_file_logger = System::new(file_logger);
    system_with_file_logger.do_something();

    // Use ConsoleLogger
    let console_logger = ConsoleLogger;
    let system_with_console_logger = System::new(console_logger);
    system_with_console_logger.do_something();
}