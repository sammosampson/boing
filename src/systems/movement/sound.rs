use crate::prelude::*;

#[system(for_each)]
pub fn set_bat_movement_sounds(
    heading: &Heading,
    buffer: &mut CommandBuffer
) {
    let heading = **heading;

    if heading == Vector::down() {
        //add_bat_up_sounds(buffer)
    } else if heading == Vector::up() {
       // add_bat_down_sounds(buffer)
    }
}
