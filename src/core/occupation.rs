// Functionality for occupations such as work and school
use bevy::prelude::{Component, Changed, Query, Commands, Resource, Res, World, FromWorld};
use rand::{thread_rng, Rng};

use super::{person::{Name, spawn_random_person}, relationships::Relationships, Populatable, Randomisable};

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

// Initialisation for Occupations
impl FromWorld for Occupations {
    fn from_world(_world: &mut World) -> Self {
        // TODO: Initialise from csv
        return Occupations {
            occupations: vec![
                Occupation { kind: OccupationType::Job, name: "Riskaware".to_owned(), workers: Vec::new(), target_number: 10 },
                Occupation { kind: OccupationType::Job, name: "Amigos Cocina".to_owned(), workers: Vec::new(), target_number: 4 }
            ],
        };
    }
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

impl Populatable for Occupation {
    fn populate(&'static self, mut commands: &mut Commands, occupations: &Res<Occupations>) {
        // Add the required number of workers to get to the target number
        for _ in 0..(self.target_number - self.workers.len()) {
            spawn_random_person(&mut commands, Some(&self), Vec::new(), Vec::new(), occupations)
        }
    }
}

#[derive(Component, PartialEq)]
pub struct PersonalOccupation {
    pub occupation: &'static Occupation
}

impl Randomisable for PersonalOccupation {
    fn get_random(occupations: &Res<Occupations>) -> Self {
        // Random number generator
        let mut rng = thread_rng();
        // Get random index
        let rand_index = rng.gen_range(0..occupations.occupations.len());
        // Return occupation at this index
        return PersonalOccupation { occupation: &occupations.occupations[rand_index] };
    }
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
                // TODO: 20 should be from a resource
                relationships.friendships.push(20);
            }
        }
    }
}
