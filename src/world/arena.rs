use crate::prelude::*;

pub const BOUNDS_MIN_Y: f32 = 32.0;
pub const BOUNDS_MAX_Y: f32 = SCREEN_HEIGHT - BOUNDS_MIN_Y;

pub fn add_arena(buffer: &mut CommandBuffer) {
    buffer.push((
        Texture(TextureResources::Table), 
        Layer(0),
        Position(centre_screen()),
        WorldEntityId::from("Arena")
    ));
}

pub fn add_arena_score_effect(buffer: &mut CommandBuffer, game_timer: &GameTimer, index: PlayerIndex) {
    buffer.push((
        create_arena_score_effect_id(index),
        create_arena_score_effect_animation(game_timer, index),
        Layer(1),
        Effect,
        Position(centre_screen()),
    ));
}

fn create_arena_score_effect_id(index: PlayerIndex) -> WorldEntityId {
    format!("ArenaScore{:?}", index).into()
}

fn create_arena_score_effect_animation(game_timer: &GameTimer, index: PlayerIndex) -> Animation {
    let mut animation = create_animation(
        Duration::from_secs(3).as_secs_f32(), 
        game_timer.total_game_time());

    animation.add_frame(TextureResources::Effect(u8::from(index)));

    animation
}
