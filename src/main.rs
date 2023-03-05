use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::Resource;
use bevy_egui::EguiPlugin;
use chrono::{NaiveDateTime, NaiveDate};

use crate::ui::UIPlugin;
use crate::person::player::setup_player;

pub mod person;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UIPlugin)

        .insert_resource(CurrentDateTime(NaiveDate::from_ymd_opt(2023, 3, 5).unwrap().and_hms_opt(0, 0, 0).unwrap()))

        .add_startup_system(setup_player)

        .run();
}

// Global constants
// Current date
#[derive(Resource)]
pub struct CurrentDateTime(NaiveDateTime);