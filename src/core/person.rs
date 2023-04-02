// Functionality for people
use bevy::ecs::component::Component;
use bevy::prelude::{Bundle, Commands, Res};
use random_string::generate;

use self::birthday::Birthday;
use super::hobby::{PersonalHobbies, Hobby};
use super::hobby::club::{PersonalClubs, Club};
use super::occupation::{Occupation, PersonalOccupation, Occupations};
use super::relationships::{Relationships};
use crate::core::Randomisable;

pub mod birthday;

// Bundle of components to represent a person
#[derive(Bundle, PartialEq)]
pub struct PersonBundle {
    pub name: Name,
    pub birthday: Birthday,
    pub relationships: Relationships,
    pub occupation: PersonalOccupation,
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

// TODO randomise all components
// Functions (Not systems)
pub fn spawn_random_person(mut commands: &mut Commands, mut occupation: Option<&'static Occupation>, hobbies: Vec<Hobby>, clubs: Vec<&'static Club>, occupations: &Res<Occupations>) {
    // Characters set for generating random names
    let name_charset = "abcdefghijklmnopqrstuvwxyz";
    // Generate random name
    // TODO: Better random names
    let name = Name {first: generate(6, name_charset), last: generate(8, name_charset)};
    // Generate random birthday
    let birthday = Birthday::get_random(occupations);
    // Initialise relationships
    // TODO: Add things to this list
    let relationships = Relationships {people: Vec::new(), friendships: Vec::new()};
    // Initialise occupation
    let personal_occupation = if occupation == None {
        PersonalOccupation::get_random(occupations)
    } else {
        PersonalOccupation { occupation: occupation.unwrap() }
    };
    // Create new person
    commands.spawn(PersonBundle {
        name: name,
        birthday: birthday,
        relationships: relationships,
        occupation: personal_occupation,
        hobbies: PersonalHobbies{hobbies: hobbies.to_vec()},
        clubs: PersonalClubs{clubs}
    });
}
