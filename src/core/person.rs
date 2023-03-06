// Functionality for people
use bevy::ecs::component::Component;
use bevy::prelude::Bundle;
use chrono::NaiveDate;

pub mod player;

#[derive(Bundle)]
pub struct PersonBundle {
    pub name: Name,
    birthday: Birthday,
    relationships: Relationships,
}

// Components

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
        let age = current_date.years_since(self.date);
        if age.is_some() {
            return age.unwrap();
        } else {
            return 0;
        }
    }
}

// Struct to represent a relationship with another person
#[derive(Component)]
pub struct Relationship {
    pub person: &'static PersonBundle,
    pub friendship: u32,
}

#[derive(Component)]
pub struct Relationships {
    pub relationships: Vec<Relationship>,
}
