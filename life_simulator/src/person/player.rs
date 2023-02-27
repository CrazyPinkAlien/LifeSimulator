// Functionality specific to the player

use bevy::ecs::system::Resource;
use super::Person;

#[derive(Resource)]
pub struct Player {
    pub person: Person,
}
