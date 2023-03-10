// Functionality for relationships
use bevy::prelude::Component;

use super::person::PersonBundle;

// Components

// Struct to represent a relationships with other people
#[derive(Component, PartialEq)]
pub struct Relationships {
    pub people: Vec<&'static PersonBundle>,
    pub friendships: Vec<u32>,
}
