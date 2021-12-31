use crate::prelude::*;

pub const BASE_BALL_VELOCITY: f32 = 180.0;
pub const SLOW_BALL_VELOCITY: f32 = 200.0;
pub const MEDIUM_BALL_VELOCITY: f32 = 220.0;
pub const FAST_BALL_VELOCITY: f32 = 240.0;
pub const BALL_VELOCITY_INCREMENT: f32 = 5.0;
pub const BALL_DIAMETER: f32 = 14.0;
pub const BALL_RADIUS: f32 = BALL_DIAMETER * 0.5;

pub struct Ball;

pub fn add_ball(buffer: &mut CommandBuffer) {
    buffer.push((
        Ball,
        Texture(TextureResources::Ball), 
        Layer(2), 
        Position(centre_screen()), 
        MaximumVelocity(BASE_BALL_VELOCITY),
        Heading::from(Vector::from(Angle::from_degrees(135.0))),
        WorldEntityId::from("Ball")
    ));
}
