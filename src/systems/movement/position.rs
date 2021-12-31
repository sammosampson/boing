use crate::prelude::*;

#[system(for_each)]
pub fn set_position(
    entity_id: &WorldEntityId,
    position: &mut Position,
    next_position: &NextPosition,
    #[resource] item_renderer: &mut ItemRenderer
) {
    if let Some(item) = item_renderer.find_mut(entity_id) {
        item.set_centre_position(**next_position);
        **position = **next_position
    }
}