// logger.rs

pub trait Logger {
    fn log(&self, message: String); // Use String instead of &str
}

pub struct FileLogger;

impl Logger for FileLogger {
    fn log(&self, message: String) {
        println!("Logging to file: {}", message);
    }
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: String) {
        println!("Logging to console: {}", message);
    }
}