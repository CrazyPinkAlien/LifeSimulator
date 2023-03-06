// Core functionality for the game
use bevy::{app::App, prelude::Plugin};

use crate::core::person::player::setup_player;

pub mod person;

// Create core plugin
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
    }
}
