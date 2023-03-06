// Functionality for people
use bevy::ecs::component::Component;
use bevy::prelude::Bundle;
use chrono::NaiveDate;

use super::occupation::Occupation;
use super::relationships::Relationships;

pub mod player;

#[derive(Bundle, PartialEq)]
pub struct PersonBundle {
    pub name: Name,
    birthday: Birthday,
    pub relationships: Relationships,
    occupation: Occupation,
}

// Components

#[derive(Component, PartialEq)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Birthday {
    pub date: NaiveDate,
}

pub trait HasAge {
    fn get_age(&self, current_date: NaiveDate) -> u32;
}

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
