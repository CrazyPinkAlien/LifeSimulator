// Functionality for a club that is specific to a hobby
use bevy::prelude::{Component, Commands, Changed, Query};

use crate::core::{person::{Name, spawn_random_person}, relationships::Relationships, Populatable};
use super::Hobby;

// Components

#[derive(Component, Clone, PartialEq)]
pub struct Club {
    pub name: String,
    pub members: Vec<&'static Name>,
    pub target_number: usize,
    pub hobby: &'static Hobby
}

impl Populatable for Club {
    fn populate(&self, mut commands: Commands) {
        // Add the required number of members to get to the target number
        for i in 0..(self.target_number - self.members.len()) {
            spawn_random_person(commands, None, vec![self.hobby], vec![self])
        }
    }
}

#[derive(Component, PartialEq)]
pub struct PersonalClubs {
    clubs: Vec<&'static Club>
}

// Systems

pub fn register_club_members (query: Query<(&Name, &PersonalClubs), Changed<PersonalClubs>>) {
    // Iterate over queries
    for (name, clubs) in query.iter() {
        // Iterate over clubs
        for club in &clubs.clubs {
            if !club.members.contains(&name) {
                club.members.push(name);
            }
        }
    }
}

pub fn meet_club_members (mut query: Query<(&mut Relationships, &mut PersonalClubs), Changed<PersonalClubs>>, mut commands: Commands) {
    // For each query result
    for (relationships, clubs) in query.iter() {
        // Get clubs
        let clubs = clubs.clubs;
        // For each club
        for club in clubs {
            // If the club has too few members populate it
            if club.members.len() < club.target_number {
                club.populate(commands);
            }
            // Add all club members to our relationships
            for person in &club.members {
                // Add club member to our relationships if they aren't there already
                if !relationships.people.contains(&person) {
                    relationships.people.push(person);
                    relationships.friendships.push(40);
                }
            }
        }
    }
}
