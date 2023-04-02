// Functionality for hobbies

use bevy::prelude::Component;

pub mod club;

#[derive(Clone, PartialEq, Copy)]
pub enum Hobby {
    Knitting,
    Pubbing
}

// Components
#[derive(Component, PartialEq)]
pub struct PersonalHobbies {
    pub hobbies: Vec<Hobby>
}
