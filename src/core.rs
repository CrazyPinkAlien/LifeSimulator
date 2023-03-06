// Core functionality for the game
use bevy::{app::App, prelude::Plugin};

use crate::core::person::player::setup_player;

use self::occupation::meet_coworkers;

pub mod person;
pub mod occupation;
pub mod relationships;

// Create core plugin
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_player)

            .add_system(meet_coworkers);
    }
}
