// Functionality for people

use bevy::{ecs::component::Component, prelude::Bundle};
use chrono::NaiveDate;

pub mod player;

#[derive(Component)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Component, Copy, Clone)]
pub struct Birthday {
    pub date: NaiveDate,
}

pub trait HasAge {
    fn get_age(&self, current_date: NaiveDate) -> u32;
}

impl HasAge for Birthday {
    fn get_age(&self, current_date: NaiveDate) -> u32 {
        return current_date.years_since(self.date).unwrap();
    }
}

#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    birthday: Birthday,
}
