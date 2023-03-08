// Functionality for storing birthdays
use bevy::prelude::Component;
use chrono::NaiveDate;

// Components

// Birthday component
#[derive(Component, Copy, Clone, PartialEq)]
pub struct Birthday {
    pub date: NaiveDate,
}

// Trait to get the age of an entity
pub trait HasAge {
    fn get_age(&self, current_date: NaiveDate) -> u32;
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