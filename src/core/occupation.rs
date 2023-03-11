// Functionality for occupations such as work and school
use bevy::prelude::{Component, Changed, Query, Commands, Resource, Res};
use rand::{thread_rng, Rng};

use super::{person::{Name, spawn_random_person}, relationships::Relationships, Populatable};

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


// Resources

#[derive(Resource)]
pub struct Occupations {
    pub occupations: Vec<Occupation>
}
// Components

// Struct to store information about an occupation
#[derive(Component, PartialEq)]
pub struct Occupation {
    pub kind: OccupationType,
    pub name: String,
    workers: Vec<&'static Name>,
    target_number: usize
}

#[derive(Component, PartialEq)]
pub struct PersonalOccupation {
    pub occupation: &'static Occupation
}

impl Populatable for Occupation {
    fn populate(&self, mut commands: &mut Commands, occupations: Res<Occupations>) {
        // Add the required number of workers to get to the target number
        for _i in 0..(self.target_number - self.workers.len()) {
            spawn_random_person(&mut commands, Some(&self), &Vec::new(), Vec::new(), &occupations)
        }
    }
}

// Functions (Not systems)
pub fn random_occupation(occupations: &Res<Occupations>) -> &'static Occupation {
    // Random number generator
    let mut rng = thread_rng();
    // Get random index
    let rand_index = rng.gen_range(0..occupations.occupations.len());
    // Return occupation at this index
    return &occupations.occupations[rand_index];
}

// Initial occupations
pub fn get_initial_occupations() -> Occupations {
    return Occupations {
        occupations: vec![
            Occupation { kind: OccupationType::Job, name: "Riskaware".to_owned(), workers: Vec::new(), target_number: 10 },
            Occupation { kind: OccupationType::Job, name: "Amigos Cocina".to_owned(), workers: Vec::new(), target_number: 4 }
        ],
    };
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
