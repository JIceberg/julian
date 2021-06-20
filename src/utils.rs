use std::cmp::{Ord, PartialEq, Eq, PartialOrd, Ordering};
use serde::{Serialize, Serializer};

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
        if self.year == 0 {
            return "None".to_string();
        }

        let year = {
            if self.year < 100 {
                format!("20{}", self.year)
            } else {
                self.year.to_string()
            }
        };
        let month = {
            if self.month < 10 {
                format!("0{}", self.month)
            } else {
                self.month.to_string()
            }
        };
        let day = {
            if self.day < 10 {
                format!("0{}", self.day)
            } else {
                self.day.to_string()
            }
        };

        let mut date_str = year;

        date_str.push_str(
            month.as_str()
        );
        date_str.push_str(
            day.as_str()
        );

        date_str
    }

    pub fn get_year(&self) -> Int {
        self.year
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
    None,
}

impl Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Winter => serializer.serialize_unit_variant("Season", 0, "Winter"),
            Self::Spring => serializer.serialize_unit_variant("Season", 1, "Spring"),
            Self::Summer => serializer.serialize_unit_variant("Season", 2, "Summer"),
            Self::Fall => serializer.serialize_unit_variant("Season", 3, "Fall"),
            Self::None => serializer.serialize_unit_variant("Season", 4, "None")
        }
    }
}