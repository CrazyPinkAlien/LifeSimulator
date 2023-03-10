// Functionality for people
use bevy::ecs::component::Component;
use bevy::prelude::{Bundle, Commands, Entity};
use chrono::NaiveDate;
use random_string::generate;

use self::birthday::Birthday;
use super::occupation::Occupation;
use super::relationships::{Relationships};
use crate::core::occupation::OccupationType::Unemployed;

pub mod birthday;

// Bundle of components to represent a person
#[derive(Bundle, PartialEq)]
pub struct PersonBundle {
    pub name: Name,
    pub birthday: Birthday,
    pub relationships: Relationships,
    pub occupation: Occupation,
}

// Components

// Component to store a person's name
#[derive(Component, PartialEq)]
pub struct Name {
    pub first: String,
    pub last: String,
}

// Functions (Not Systems)

pub fn spawn_random_person(commands: &mut Commands) -> Entity {
    let name_charset = "abcdefghijklmnopqrstuvwxyz";
    let name = Name {first: generate(6, name_charset), last: generate(8, name_charset)};
    return commands.spawn( PersonBundle {
        name: name,
        birthday: Birthday { date: NaiveDate::from_ymd_opt(1995, 6, 16).unwrap()},
        relationships: Relationships {people: Vec::new(), friendships: Vec::new()},
        occupation: Occupation {kind: Unemployed, name: "N/A".to_owned(), workers: Vec::new()}
    }).id();
}
