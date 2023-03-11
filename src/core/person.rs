// Functionality for people
use bevy::ecs::component::Component;
use bevy::prelude::{Bundle, Commands};
use random_string::generate;

use self::birthday::{Birthday, random_birthday};
use super::hobby::{PersonalHobbies, Hobby};
use super::hobby::club::{PersonalClubs, Club};
use super::occupation::{Occupation, random_occupation};
use super::relationships::{Relationships};

pub mod birthday;

// Bundle of components to represent a person
#[derive(Bundle, PartialEq)]
pub struct PersonBundle {
    pub name: Name,
    pub birthday: Birthday,
    pub relationships: Relationships,
    pub occupation: Occupation,
    pub hobbies: PersonalHobbies,
    pub clubs: PersonalClubs
}

// Components

// Component to store a person's name
#[derive(Component, PartialEq)]
pub struct Name {
    pub first: String,
    pub last: String,
}

// Functions (Not systems)
pub fn spawn_random_person(mut commands: Commands, occupation: Option<Occupation>, hobbies: Vec<&Hobby>, clubs: Vec<&Club>) {
    // Characters set for generating random names
    let name_charset = "abcdefghijklmnopqrstuvwxyz";
    // Generate random name
    // TODO: Better random names
    let name = Name {first: generate(6, name_charset), last: generate(8, name_charset)};
    // Generate random birthday
    let birthday = random_birthday();
    // Initialise relationships
    // TODO: Add things to this list
    let relationships = Relationships {people: Vec::new(), friendships: Vec::new()};
    // Initialise occupation
    if occupation == None  {
        occupation = Some(random_occupation())
    }
    // Create new person
    commands.spawn(PersonBundle {
        name: name,
        birthday: birthday,
        relationships: relationships,
        occupation: occupation.unwrap(),
        hobbies: PersonalHobbies {hobbies},
        clubs: PersonalClubs{clubs}
    });
}
