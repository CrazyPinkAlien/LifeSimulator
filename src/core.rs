// Core functionality for the game
use bevy::{app::App, prelude::{Plugin, Commands}};
use chrono::NaiveDate;

use crate::core::player::setup_player;
use crate::core::hobby::get_initial_hobbies;

use self::{occupation::meet_coworkers, time::CurrentDateTime, hobby::{Hobbies, Hobby, club::{Club, meet_club_members, register_club_members}}};

pub mod person;
pub mod occupation;
pub mod relationships;
pub mod player;
pub mod time;
pub mod hobby;

// Create core plugin
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentDateTime(NaiveDate::from_ymd_opt(2023, 3, 5).unwrap().and_hms_opt(0, 0, 0).unwrap()))
            .insert_resource(get_initial_hobbies())

            .add_startup_system(setup_player)

            .add_system(meet_coworkers)
            .add_system(meet_club_members)
            .add_system(register_club_members);
    }
}

// Trait to define something that can be populated
pub trait Populatable {
    fn populate(&self, commands: Commands);
}
