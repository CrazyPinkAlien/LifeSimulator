use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

use crate::ui::UIPlugin;
use crate::core::CorePlugin;

pub mod core;
pub mod ui;

// Main entry point
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(CorePlugin)

        .run();
}
