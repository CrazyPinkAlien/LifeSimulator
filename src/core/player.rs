// Functionality specific to the player
use bevy::prelude::{Bundle, Component, Commands};
use chrono::NaiveDate;

use crate::core::occupation::{Occupation, OccupationType::*};
use crate::core::person::{PersonBundle, Name};
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
pub fn setup_player(mut commands: Commands) {
    commands.spawn(
        PersonBundle {
            name: Name {first: "James".to_string(), last: "Wrigley".to_string()},
            birthday: Birthday { date: NaiveDate::from_ymd_opt(1993, 11, 1).unwrap()},
            relationships: Relationships {people: Vec::new(), friendships: Vec::new()},
            occupation: Occupation {kind: Unemployed, name: "N/A".to_owned(), workers: Vec::new()}
        }
    );
    commands.spawn(
        PlayerBundle {
            _p: Player,
            person: PersonBundle{
                name: Name {first: "Emily".to_string(), last: "Tyler".to_string()},
                birthday: Birthday { date: NaiveDate::from_ymd_opt(1995, 6, 16).unwrap()},
                relationships: Relationships {people: Vec::new(), friendships: Vec::new()},
                occupation: Occupation {kind: Unemployed, name: "N/A".to_owned(), workers: Vec::new()}
            },
        }
    );
}
