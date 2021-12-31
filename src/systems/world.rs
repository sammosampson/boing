use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Remove>())]
pub fn remove_entity(
    entity: &Entity,
    id: &WorldEntityId,
    #[resource] renderer: &mut ItemRenderer,
    buffer: &mut CommandBuffer,
) {
    if renderer.contains(id) {
        println!("remove from render {:?}", id);
        renderer.remove_item_to_render(id);
    }

    println!("remove entity {:?}", id);
    buffer.remove(*entity);
}   
