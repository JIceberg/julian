use std::cmp::{Ord, PartialEq, Eq, PartialOrd, Ordering};

pub type Int = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Date {
    year: Int,
    month: Int,
    day: Int,
}

impl Date {
    pub fn new(year: Int, month: Int, day: Int) -> Self {
        Self { year, month, day }
    }

    pub fn as_fuzzy(&self) -> String {
        let mut date_str = self.year.to_string();

        date_str.push_str(
            self.month
                .to_string()
                .as_str()
        );
        date_str.push_str(
            self.day
                .to_string()
                .as_str()
        );

        date_str
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_fuzzy().cmp(&(other.as_fuzzy()))
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Season {
    Fall,
    Spring,
    Summer,
    Winter,
}