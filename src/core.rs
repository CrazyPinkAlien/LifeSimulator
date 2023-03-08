// Core functionality for the game
use bevy::{app::App, prelude::Plugin};
use chrono::NaiveDate;

use crate::core::player::setup_player;

use self::{occupation::meet_coworkers, time::CurrentDateTime};

pub mod person;
pub mod occupation;
pub mod relationships;
pub mod player;
pub mod time;

// Create core plugin
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentDateTime(NaiveDate::from_ymd_opt(2023, 3, 5).unwrap().and_hms_opt(0, 0, 0).unwrap()))

            .add_startup_system(setup_player)

            .add_system(meet_coworkers);
    }
}
