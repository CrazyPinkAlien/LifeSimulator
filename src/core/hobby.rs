// Functionality for hobbies
use bevy::prelude::{Component, Resource};

use self::club::Club;

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
    let mut pubbing = Hobby {
        name: "Pubbing".to_owned(),
        clubs: Vec::new()
    };
    let mut pubbing_clubs = vec![
        Club {name:"Lion's Head".to_owned(), target_number:10, hobby: &pubbing, members: Vec::new() },
        Club {name:"Fancy Cocktails".to_owned(), target_number: 6, hobby: &pubbing, members: Vec::new() }
    ];
    pubbing.clubs.append(&mut pubbing_clubs);
    let knitting = Hobby {
        name: "Knitting".to_owned(),
        clubs: Vec::new()
    };
    return Hobbies {hobbies: vec![pubbing, knitting]};
}
