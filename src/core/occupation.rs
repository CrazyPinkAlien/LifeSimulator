// Functionality for occupations such as work and school
use bevy::prelude::{Component, Changed, Query};

use super::{person::PersonBundle, relationships::Relationships};

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
    pub workers: Vec<&'static PersonBundle>,
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