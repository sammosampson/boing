use crate::prelude::*;

pub const BAT_POSITION_OFFSET: f32 = 40.0;
pub const BAT_HEIGHT: f32 = 128.0;
pub const BAT_WIDTH: f32 = 18.0;
pub const HALF_BAT_HEIGHT: f32 = BAT_HEIGHT * 0.5;
pub const HALF_BAT_WIDTH: f32 = BAT_WIDTH * 0.5;
pub const MAXIMUM_BAT_VELOCITY: f32 = 200.0;

pub fn add_bat_up_sounds(buffer: &mut CommandBuffer) {
    buffer.push(create_sound_components(SoundResources::Up));   
}

pub fn add_bat_down_sounds(buffer: &mut CommandBuffer) {
    buffer.push(create_sound_components(SoundResources::Down));   
}

pub fn add_bat_hit_sounds(buffer: &mut CommandBuffer, ball_speed: f32) {
    buffer.push(create_sound_components(SoundResources::Hit(random_number(0..4))));   
    buffer.push(create_sound_components(get_bat_hit_sound(ball_speed)));   
}

pub fn get_bat_hit_sound(ball_speed: f32) -> SoundResources {
    if ball_speed <= SLOW_BALL_VELOCITY {
       return SoundResources::HitSlow;
    } 
    if ball_speed <= MEDIUM_BALL_VELOCITY {
        return SoundResources::HitMedium;
    } 
    if ball_speed <= FAST_BALL_VELOCITY {   
        return SoundResources::HitFast;
    }
    SoundResources::HitVeryFast
}

#[derive(Debug, Copy, Clone)]
pub struct Bat(PlayerIndex);

impl Deref for Bat {
    type Target = PlayerIndex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn set_bat_score_texture(buffer: &mut CommandBuffer, bat_entity: Entity, index: PlayerIndex) {
    let texture = if index == PlayerIndex::Player1 { TextureResources::Bat0(2) } else { TextureResources::Bat1(2) };
    set_texture(buffer, bat_entity, texture);
}

pub fn set_normal_bat_texture(buffer: &mut CommandBuffer, bat_entity: Entity, index: PlayerIndex) {
    let texture = if index == PlayerIndex::Player1 { TextureResources::Bat0(0) } else { TextureResources::Bat1(0) };
    set_texture(buffer, bat_entity, texture);
}

pub fn set_bat_hit_effect(buffer: &mut CommandBuffer, game_timer: &GameTimer, bat_entity: Entity, index: PlayerIndex) {
    buffer.add_component(bat_entity, create_bat_hit_effect_animation(game_timer, index));
}

fn create_bat_hit_effect_animation(game_timer: &GameTimer, index: PlayerIndex) -> Animation {
    let mut animation = create_animation(
        Duration::from_secs(1).as_secs_f32(), 
        game_timer.total_game_time());

    if index == PlayerIndex::Player1 {
        animation.add_frame(TextureResources::Bat0(1));
        animation.add_frame(TextureResources::Bat0(0));
    } else {        
        animation.add_frame(TextureResources::Bat1(1));
        animation.add_frame(TextureResources::Bat1(0));
    }

    animation
}

pub fn add_player_one_bat(buffer: &mut CommandBuffer) {
    add_bat(buffer,PlayerIndex::Player1, TextureResources::Bat0(0),BAT_POSITION_OFFSET,"Player1");
}

pub fn add_player_two_bat(buffer: &mut CommandBuffer) {
    add_bat(buffer,PlayerIndex::Player2, TextureResources::Bat1(0),SCREEN_WIDTH - BAT_POSITION_OFFSET,"Player2");
}

fn add_bat(buffer: &mut CommandBuffer, index: PlayerIndex, texture: TextureResources, x: f32, name: &str) {
    buffer.push((
        Bat(index),
        Texture(texture), 
        Layer(2), 
        Position(Vector::new(x, HALF_SCREEN_HEIGHT)), 
        MaximumVelocity(MAXIMUM_BAT_VELOCITY),
        Heading::default(),
        WorldEntityId::from(name),
        Player
    ));
}
