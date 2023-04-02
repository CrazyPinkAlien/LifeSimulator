// Core functionality for the game
use bevy::{app::App, prelude::{Plugin, Commands, Res}};

use crate::core::player::setup_player;

use self::{occupation::{meet_coworkers, Occupations}, hobby::club::Clubs};
use self::time::CurrentDateTime;
use self::hobby::club::meet_club_members;

pub mod person;
pub mod occupation;
pub mod relationships;
pub mod player;
pub mod time;
pub mod hobby;

// Create core plugin
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentDateTime>()
            .init_resource::<Clubs>()
            .init_resource::<Occupations>()

            .add_startup_system(setup_player)

            .add_system(meet_coworkers)
            .add_system(meet_club_members);
    }
}

// Trait to define something that can be populated
pub trait Populatable {
    fn populate(&self, commands: &mut Commands, occupations: &Res<Occupations>);
}

// Something that can create a random value of itself
pub trait Randomisable {
    fn get_random(occupations: &Res<Occupations>) -> Self;
}