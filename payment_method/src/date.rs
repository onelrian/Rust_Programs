#[derive(Debug)]
pub struct Date {
    year: u128,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u128, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }

    pub fn year(&self) -> u128 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}