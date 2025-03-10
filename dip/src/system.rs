// system.rs

use crate::logger::Logger;

pub struct System<T: Logger> {
    logger: T,
}

impl<T: Logger> System<T> {
    pub fn new(logger: T) -> Self {
        System { logger }
    }

    pub fn do_something(&self) {
        self.logger.log("System is doing something!".to_string()); // Convert &str to String
    }
}