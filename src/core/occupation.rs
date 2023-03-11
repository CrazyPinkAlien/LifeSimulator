// Functionality for occupations such as work and school
use bevy::prelude::{Component, Changed, Query, Commands};

use super::{person::Name, relationships::Relationships, Populatable};
use crate::core::occupation::OccupationType::Unemployed;

// Enum for occupation types
#[derive(PartialEq)]
pub enum OccupationType {
    PrimarySchool,
    SecondarySchool,
    University,
    Job,
    Unemployed,
    Retired,
}

// Components

// Struct to store information about an occupation
#[derive(Component, PartialEq)]
pub struct Occupation {
    pub kind: OccupationType,
    pub name: String,
    pub workers: Vec<&'static Name>,
}

impl Populatable for Occupation {
    fn populate(&self, mut commands: Commands) {
        // TODO: implement
    }
}

// Functions (Not systems)
pub fn random_occupation() -> Occupation {
    // TODO: Make this random
    return Occupation {kind: Unemployed, name: "N/A".to_owned(), workers: Vec::new()};
}

// Systems

// System to ensure that when a person changes occupation they meet their new coworkers
pub fn meet_coworkers (mut query: Query<(&mut Relationships, &Occupation), Changed<Occupation>>) {
    // Loop over matching entities
    for (mut relationships, occupation) in query.iter_mut() {
        // Loop over coworkers
        for person in &occupation.workers {
            // Add our coworker to our relationships if they aren't there already
            if !relationships.people.contains(&person) {
                relationships.people.push(person);
                relationships.friendships.push(20);
            }
        }
    }
}
