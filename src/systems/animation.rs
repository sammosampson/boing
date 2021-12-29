use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<Texture>())]
pub fn render_first_animation_frame(
    entity: &Entity,
    entity_id: &WorldEntityId,
    animation: &Animation,
    buffer: &mut CommandBuffer
) {
    println!("animating first frame {:?}", entity_id);
    let frame_texture = animation.get_frame_texture(0);
    set_texture(buffer, *entity, frame_texture);
}

#[system(for_each)]
pub fn render_animation_frame(
    entity: &Entity,
    entity_id: &WorldEntityId,
    animation: &Animation,
    texture: &Texture,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    println!("animating {:?}", entity_id);
    if let Some(frame) = animation.get_frame(game_timer.total_game_time()) {
        let frame_texture = animation.get_frame_texture(frame);
        if texture.0 == frame_texture {
            return;
        }
        set_texture(buffer, *entity, frame_texture);
    } else {
        remove_animation(buffer, *entity);
    }
}