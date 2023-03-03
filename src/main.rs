use bevy::app::App;
use bevy::DefaultPlugins;

use crate::ui::UIPlugin;
use crate::person::player::setup_player;

pub mod person;
pub mod ui;

// States
#[derive(Clone, Hash, Debug, Eq, PartialEq)]
enum AppState {
    MainMenu,
    PlayerInfo,
}

fn main() {
    App::new()
        .add_state(AppState::MainMenu)

        .add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)

        .add_startup_system(setup_player)

        .run();
}