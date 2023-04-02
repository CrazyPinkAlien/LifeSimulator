// Functionality specific to the player
use bevy::prelude::{Bundle, Component, Commands, Res};
use chrono::NaiveDate;

use crate::core::person::{PersonBundle, Name};
use super::Randomisable;
use super::hobby::PersonalHobbies;
use super::hobby::club::PersonalClubs;
use super::occupation::{PersonalOccupation, Occupations};
use super::person::birthday::Birthday;
use super::relationships::Relationships;

// Bundle of components that comprise a player
#[derive(Bundle)]
struct PlayerBundle {
    _p: Player,
    person: PersonBundle,
}

// Components

// Tag to identify the player
#[derive(Component)]
pub struct Player;

// Startup Systems

// Setup function for the player
pub fn setup_player(mut commands: Commands, occupations: Res<Occupations>) {
    commands.spawn(
        PlayerBundle {
            _p: Player,
            person: PersonBundle {
                name: Name {first: "Emily".to_string(), last: "Tyler".to_string()},
                birthday: Birthday { date: NaiveDate::from_ymd_opt(1995, 6, 16).unwrap()},
                relationships: Relationships {people: Vec::new(), friendships: Vec::new()},
                occupation: PersonalOccupation::get_random(&occupations),
                hobbies: PersonalHobbies { hobbies: Vec::new() },
                clubs: PersonalClubs { clubs: Vec::new() }
            },
        }
    );
}
