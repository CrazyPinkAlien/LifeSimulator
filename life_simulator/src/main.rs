use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::Camera2dBundle;
use bevy::prelude::Color;
use bevy::prelude::Commands;
use bevy::prelude::TextBundle;
use bevy::text::TextAlignment;
use bevy::ui::PositionType;
use bevy::ui::Style;
use bevy::text::TextStyle;
use bevy::ui::UiRect;
use bevy::ecs::system::Res;
use bevy::asset::AssetServer;
use bevy::ui::Val;

use crate::person::player::Player;
use crate::person::Person;

pub mod person;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_ui)
        .insert_resource(Player {person: { Person {first_name: String::from("Emily"), last_name: String::from("Tyler")}}} )
        .run();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, player: Res<Player>) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            &player.person.first_name,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        }),
    ));
}
