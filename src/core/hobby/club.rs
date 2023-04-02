// Functionality for a club that is specific to a hobby
use bevy::prelude::{Component, Commands, Changed, Query, Res, FromWorld, World, Resource};

use crate::core::{person::{spawn_random_person, Name}, relationships::Relationships, Populatable, occupation::Occupations};

use super::Hobby;
use crate::core::hobby::Hobby::Pubbing;

// Resources
#[derive(Resource)]
pub struct Clubs {
    pub clubs: Vec<Club>
}

// Initialisation for Clubs
impl FromWorld for Clubs {
    fn from_world(_world: &mut World) -> Self {
        let pubbing_clubs = vec![
            Club {name:"Lion's Head".to_owned(), target_number:10, hobby: Pubbing, members: Vec::new() },
            Club {name:"Fancy Cocktails".to_owned(), target_number: 6, hobby: Pubbing, members: Vec::new() }
        ];
        return Clubs { clubs: pubbing_clubs.to_owned() }
    }
}

// Components

#[derive(Component, Clone, PartialEq)]
pub struct Club {
    pub name: String,
    hobby: Hobby,
    target_number: usize,
    members: Vec<&'static Name>
}

impl Populatable for Club {
    fn populate(&self, mut commands: &mut Commands, occupations: &Res<Occupations>) {
        // Add the required number of members to get to the target number
        for _ in 0..(self.target_number - self.members.len()) {
            spawn_random_person(&mut commands, None, vec![self.hobby], vec![&self], occupations)
        }
    }
}

#[derive(Component, PartialEq)]
pub struct PersonalClubs {
    pub clubs: Vec<&'static Club>
}

// Systems

pub fn meet_club_members (mut query: Query<(&mut Relationships, &mut PersonalClubs), Changed<PersonalClubs>>, mut commands: Commands, occupations: Res<Occupations>) {
    // For each query result
    for (mut relationships, clubs) in query.iter_mut() {
        // Get clubs
        let clubs = &clubs.clubs;
        // For each club
        for club in clubs {
            // If the club has too few members populate it
            if club.members.len() < club.target_number {
                club.populate(&mut commands, &occupations);
            }
            // Add all club members to our relationships
            for person in &club.members {
                // Add club member to our relationships if they aren't there already
                if !relationships.people.contains(&person) {
                    relationships.people.push(person);
                    // TODO: 40 should be in a resource
                    relationships.friendships.push(40);
                }
            }
        }
    }
}
