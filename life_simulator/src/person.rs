// Functionality for people

use bevy::ecs::component::Component;

pub mod player;

#[derive(Component)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
