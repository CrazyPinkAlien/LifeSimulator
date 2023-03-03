// Functionality specific to the player

use bevy::prelude::{Bundle, Component, Commands};
use super::{PersonBundle, Name, Age};

// Tag to identify the player
#[derive(Component)]
pub struct Player;

// Bundle of components that comprise a player
#[derive(Bundle)]
struct PlayerBundle {
    person: PersonBundle,
    _p: Player
}

// Setup function for the player
pub fn setup_player(mut commands: Commands) {
    commands.spawn(
        PlayerBundle {
            person: PersonBundle{ name: Name {first: "Emily".to_string(), last: "Tyler".to_string()}, age: Age {age: 27}},
            _p: Player
        }
    );
}
