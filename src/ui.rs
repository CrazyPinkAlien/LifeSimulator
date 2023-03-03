//UI functonality

use bevy::prelude::{Plugin, App, Commands, AssetServer, Res, Camera2dBundle, Component, Query, With, Changed, Color, default, BuildChildren};
use bevy::text::{Text, TextStyle};
use bevy::ui::{Style, Size, Val, UiRect, Interaction, BackgroundColor, JustifyContent, AlignItems};
use bevy::ui::node_bundles::{ButtonBundle, TextBundle};
use bevy::ui::widget::Button;
use bevy::ecs::schedule::{SystemSet, State};
use bevy::ecs::system::ResMut;

use crate::AppState;
use crate::person::{Name, Age};
use crate::person::player::Player;

// Create UI plugin
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_ui)
        .add_startup_system(setup_player_name)

        .add_system(update_player_name)
        .add_system(button_system)
        .add_system(player_name_button)

        .add_system_set(
            SystemSet::on_update(AppState::PlayerInfo)
                .with_system(display_player_age)
        );
    }
}

// Function to setup the UI
fn setup_ui(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct PlayerNameText;

// Function to write the player name at the top of the screen
fn setup_player_name(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn( ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            }
        ), PlayerNameText));
    });
}

// Function to update the player name whereever it is used
fn update_player_name(player_query: Query<&Name, (Changed<Name>, With<Player>)>, mut text_query: Query<&mut Text, With<PlayerNameText>>) {
    if !player_query.is_empty() {
        // Get player name
        let player_name = player_query.single();
        // Update text
        for mut text in &mut text_query.iter_mut() {
            text.sections[0].value = player_name.first.clone();
        }
    }
}

// Player name button pressed
fn player_name_button(mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<Button>, With<PlayerNameText>)>, mut app_state: ResMut<State<AppState>>) {
    if !interaction_query.is_empty() {
        let interaction = interaction_query.single_mut();
        if *interaction == Interaction::Clicked {
            app_state.set(AppState::PlayerInfo).unwrap();
        }
    }
}

// Button colours definitions
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Colour system for all buttons
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
struct PlayerAgeText;

fn display_player_age(mut commands: Commands, asset_server: Res<AssetServer>, age_query: Query<&Age, With<Player>>) {
    if !age_query.is_empty() {
        let age = age_query.single();
        commands.spawn((TextBundle::from_section(
            age.age.to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            }
        ), PlayerAgeText));
    }
}