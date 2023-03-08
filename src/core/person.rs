// Functionality for people
use bevy::ecs::component::Component;
use bevy::prelude::Bundle;

use self::birthday::Birthday;
use super::occupation::Occupation;
use super::relationships::Relationships;

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
