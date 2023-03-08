// Functionality for tracking and progressing time

use bevy::prelude::Resource;
use chrono::NaiveDateTime;

// Current date and time
#[derive(Resource)]
pub struct CurrentDateTime(pub NaiveDateTime);
