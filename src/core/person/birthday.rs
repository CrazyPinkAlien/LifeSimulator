// Functionality for storing birthdays
use bevy::prelude::{Component, Res};
use chrono::NaiveDate;

use crate::core::{Randomisable, occupation::Occupations};

// Trait to get the age of an entity
pub trait HasAge {
    fn get_age(&self, current_date: NaiveDate) -> u32;
}

// Components

// Birthday component
#[derive(Component, Copy, Clone, PartialEq)]
pub struct Birthday {
    pub date: NaiveDate,
}

// Implementation of HasAge for the Birthday component
impl HasAge for Birthday {
    fn get_age(&self, current_date: NaiveDate) -> u32 {
        let age = current_date.years_since(self.date);
        if age.is_some() {
            return age.unwrap();
        } else {
            return 0;
        }
    }
}

impl Randomisable for Birthday {
    fn get_random(_occupations: &Res<Occupations>) -> Self {
        // TODO: Make this random
        return Birthday { date: NaiveDate::from_ymd_opt(1995, 6, 16).unwrap()};
    }
}
