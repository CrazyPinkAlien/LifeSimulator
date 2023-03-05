// UI functonality
use bevy::app::{App, Plugin};
use bevy::ecs::system::{ResMut, Res};
use bevy::prelude::{Resource, With, Query, IntoSystemDescriptor};
use bevy_egui::EguiContext;
use bevy_egui::egui::{CentralPanel, SidePanel, DragValue};
use bevy_egui::egui::widgets::Button;

use crate::person::{Name, Age};
use crate::person::player::Player;

// UI States
#[derive(PartialEq)]
enum UIState {
    MainMenu,
    PlayerInfo,
}

// Current UI state
#[derive(Resource, PartialEq)]
struct CurrentUIState(UIState);

// Create UI plugin
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentUIState(UIState::MainMenu))
            .add_system(left_side_menu)
            .add_system(main_menu_ui.after(left_side_menu))
            .add_system(player_info_ui.after(left_side_menu));
    }
}

// Left hand side menu
fn left_side_menu(mut ui_state: ResMut<CurrentUIState>, mut egui_context: ResMut<EguiContext>) {
    SidePanel::left("side_panel").default_width(200.0).show(egui_context.ctx_mut(), |ui|  {
        if ui.add(Button::new("Main Menu")).clicked() {
            ui_state.0 = UIState::MainMenu;
        }
        if ui.add(Button::new("Player Info")).clicked() {
            ui_state.0 = UIState::PlayerInfo;
        }
    });
}

// UI for the main menu
fn main_menu_ui(ui_state: Res<CurrentUIState>, mut egui_context: ResMut<EguiContext>, player_query: Query<(&Name, &Age), With<Player>>) {
    if ui_state.0 == UIState::MainMenu && !player_query.is_empty() {
        // Get player info
        let player_info = player_query.single();
        let player_name = player_info.0;
        let player_age = player_info.1;
        // Draw UI
        CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ui.heading("Main Menu");
            ui.label(format!("{} {}", player_name.first.clone(), player_name.last.clone()));
            ui.label(format!("Age: {}", player_age.age.to_string()));
        });
    }
}

// UI showing player info
fn player_info_ui(ui_state: Res<CurrentUIState>, mut egui_context: ResMut<EguiContext>, mut player_query: Query<(&mut Name, &mut Age), With<Player>>) {
    if ui_state.0 == UIState::PlayerInfo && !player_query.is_empty() {
        // Get player info
        let player_info = player_query.single_mut();
        let mut player_name = player_info.0;
        let mut player_age = player_info.1;
        // Draw UI
        CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ui.heading("Player Info");
            ui.label("First Name:");
            ui.text_edit_singleline(&mut player_name.first);
            ui.label("Last Name:");
            ui.text_edit_singleline(&mut player_name.last);
            ui.label("Age:");
            ui.add(DragValue::new(&mut player_age.age));
        });
    }
}