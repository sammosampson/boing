use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Bat>())]
pub fn contain_bat_in_bounds(
    entity: &Entity,
    position: &Position,
    heading: &mut Heading,
    buffer: &mut CommandBuffer
) {
    if position.y + HALF_BAT_HEIGHT > BOUNDS_MAX_Y + 10.0 && heading.y > 0.0 {
        **heading = Vector::default();
        buffer.remove_component::<Direction>(*entity);
    }

    if position.y - HALF_BAT_HEIGHT < BOUNDS_MIN_Y - 10.0 && heading.y < 0.0 {
        **heading = Vector::default();
        buffer.remove_component::<Direction>(*entity);
    }
}