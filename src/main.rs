use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::Resource;
use bevy_egui::EguiPlugin;
use chrono::{NaiveDateTime, NaiveDate};

use crate::ui::UIPlugin;
use crate::core::CorePlugin;

pub mod core;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(CorePlugin)

        .insert_resource(CurrentDateTime(NaiveDate::from_ymd_opt(2023, 3, 5).unwrap().and_hms_opt(0, 0, 0).unwrap()))

        .run();
}

// Global constants
// Current date
#[derive(Resource)]
pub struct CurrentDateTime(NaiveDateTime);
