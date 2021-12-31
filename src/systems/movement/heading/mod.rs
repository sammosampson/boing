
mod computer_controlled;
mod human_controlled;
pub use computer_controlled::*;
pub use human_controlled::*;

use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Bat>())]
pub fn set_bat_heading_from_direction(direction: &Direction, heading: &mut Heading) {
    match direction {
        Direction::Up => **heading = Vector::up(),
        Direction::Down => **heading = Vector::down(),
    }
}

#[system(for_each)]
#[filter(component::<Bat>())]
#[filter(!component::<Direction>())]
pub fn reset_bat_heading_with_no_direction(heading: &mut Heading) {
    **heading = Vector::default();
}
