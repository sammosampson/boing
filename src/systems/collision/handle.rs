use crate::prelude::*;

#[system(for_each)]
#[filter(component::<WallCollision>())]
pub fn handle_wall_collision(
    entity: &Entity,
    position: &mut Position,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    **heading = heading.to_y_inverted();
    buffer.push(create_impact(game_timer, *position));
    buffer.push(create_wall_impact_sound());
    buffer.remove_component::<WallCollision>(*entity);    
}

#[system(for_each)]
#[filter(component::<BatCollision>())]
pub fn handle_bat_collision(
    entity: &Entity,
    position: &mut Position,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    **heading = heading.to_x_inverted();
    buffer.push(create_impact(game_timer, *position));
    buffer.remove_component::<BatCollision>(*entity);    
}