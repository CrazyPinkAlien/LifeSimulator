// Functionality for people

use bevy::{ecs::component::Component, prelude::Bundle};

pub mod player;

#[derive(Component)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Component)]
pub struct Age {
    pub age: u32,
}

#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    age: Age,
}
