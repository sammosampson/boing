use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Player>())]
#[filter(component::<Bat>())]
#[filter(component::<Direction>())]
pub fn play_bat_movement_sounds(direction: &Direction, heading: &Heading, buffer: &mut CommandBuffer) {
    match direction {
        Direction::Up =>
            if **heading != Vector::up() {
                add_bat_up_sounds(buffer);
            },
        Direction::Down => 
            if **heading != Vector::down() {
                add_bat_down_sounds(buffer);
            },
    }
}
