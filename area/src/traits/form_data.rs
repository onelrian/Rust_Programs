use std::io::Error;

// use super::area::Area;


pub trait FormData {

    fn new() -> Self;
    fn  collect_data(&mut self) -> Result<(), Error>;
}