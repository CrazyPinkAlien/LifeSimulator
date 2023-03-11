// Functionality for hobbies

// Functionality for hobbies
use bevy::prelude::{Component, Resource};

use self::club::Club;

use super::HasInitialValues;

pub mod club;

// Components

#[derive(Component, Clone, PartialEq)]
pub struct Hobby {
    pub name: String,
    pub clubs: Vec<Club>,
}

#[derive(Component, PartialEq)]
pub struct PersonalHobbies {
    pub hobbies: Vec<&'static Hobby>
}

// Resources

#[derive(Resource)]
pub struct Hobbies {
    pub hobbies: Vec<Hobby>
}

// Initial hobbies
pub fn get_initial_hobbies() -> Hobbies {
    // TODO: Link the clubs to their hobbies
    return Hobbies {hobbies: vec![
        Hobby {
            name: "Pubbing".to_owned(),
            clubs: vec![
                Club {name:"Lion's Head".to_owned(),members:Vec::new(), target_number: 10, hobby: this },
                Club {name:"Fancy Cocktails".to_owned(),members:Vec::new(), target_number: 6, hobby: this }
            ]
        },
        Hobby {
            name: "Knitting".to_owned(),
            clubs: Vec::new()
        }
    ]};
}
