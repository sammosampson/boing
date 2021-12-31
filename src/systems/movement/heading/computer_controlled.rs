use crate::prelude::*;

const TARGET_POSITION_THRESHOLD: f32 = 10.0;

#[system(simple)]
#[read_component(Bat)]
#[read_component(Ball)]
#[read_component(Position)]
#[read_component(Heading)]
#[write_component(Direction)]
pub fn calculate_computer_player_two_direction(
    world: &SubWorld,
    buffer: &mut CommandBuffer
) {
    let (bat_entity, bat_position) = get_bat(world);

    let target_y_1 = HALF_SCREEN_HEIGHT;
    let mut target_y = target_y_1;

    if let Some((ball_position, ball_heading)) = get_ball_position(world) {
        if ball_heading.x > 0.0 {
            let target_y_2 = ball_position.y;

            let x_distance = (ball_position.x - bat_position.x).abs();
            let weight_1 = min_float(1.0, x_distance / HALF_SCREEN_WIDTH);
            let weight_2 = 1.0 - weight_1;

            target_y = (weight_1 * target_y_1) + (weight_2 * target_y_2);
        }
    }
    
    if bat_position.y < target_y - TARGET_POSITION_THRESHOLD {
        buffer.add_component(bat_entity, Direction::Up)
    } else if bat_position.y > target_y + TARGET_POSITION_THRESHOLD {
        buffer.add_component(bat_entity, Direction::Down)
    } else {
        buffer.remove_component::<Direction>(bat_entity)
    }
}

fn get_bat(world: &SubWorld) -> (Entity, Vector) {
    <(Entity, &Position, &Bat)>::query()
        .iter(world)
        .filter(|(_, __, bat)| ***bat == PlayerIndex::Player2)
        .map(|(entity, position, _)| (*entity, **position))
        .nth(0)
        .expect("No bat found")       
}

fn get_ball_position(world: &SubWorld) -> Option<(Vector, Vector)> {
    <(&Position, &Heading)>::query()
        .filter(component::<Ball>())
        .iter(world)
        .map(|(position, heading)| (**position, **heading))
        .nth(0)
}

fn min_float(first: f32, second: f32) -> f32 {
    if first <= second {
        return first;
    }
    second
}