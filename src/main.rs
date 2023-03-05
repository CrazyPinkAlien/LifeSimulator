use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

use crate::ui::UIPlugin;
use crate::person::player::setup_player;

pub mod person;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UIPlugin)

        .add_startup_system(setup_player)

        .run();
}