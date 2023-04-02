// Functionality for tracking and progressing time

use bevy::prelude::{Resource, FromWorld, World};
use chrono::{NaiveDateTime, NaiveDate};

// Current date and time
#[derive(Resource)]
pub struct CurrentDateTime(pub NaiveDateTime);

// Initialisation for current date and time
impl FromWorld for CurrentDateTime {
    fn from_world(_world: &mut World) -> Self {
        return CurrentDateTime(NaiveDate::from_ymd_opt(2023, 3, 5).unwrap().and_hms_opt(0, 0, 0).unwrap());
    }
}