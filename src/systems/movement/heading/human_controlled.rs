use crate::prelude::*;

#[system(for_each)]
#[read_component(Bat)]
#[write_component(Direction)]
pub fn set_player_one_direction_from_input(
    event: &SystemEvent,
    world: &mut SubWorld,
    buffer: &mut CommandBuffer
) {
    set_player_direction_from_input(
        event,
        world, 
        PlayerIndex::Player1, 
        VirtualKeyCode::Q, 
        VirtualKeyCode::A,
        buffer
    );
}

#[system(for_each)]
#[read_component(Bat)]
#[write_component(Direction)]
pub fn set_player_two_direction_from_input(
    event: &SystemEvent,
    world: &mut SubWorld,
    buffer: &mut CommandBuffer
) {
    set_player_direction_from_input(
        event,
        world, 
        PlayerIndex::Player2, 
        VirtualKeyCode::Up, 
        VirtualKeyCode::Down,
        buffer
    );
}

fn set_player_direction_from_input(
    event: &SystemEvent,
    world: &mut SubWorld,
    index: PlayerIndex,
    up_key: VirtualKeyCode,
    down_key: VirtualKeyCode,
    buffer: &mut CommandBuffer
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            if button.is_pressed(up_key, state) {
                set_direction(buffer, world, index, Direction::Up)
            } else if button.is_pressed(down_key, state) {
                set_direction(buffer, world, index, Direction::Down)
            } else {
                remove_direction(buffer, world, index)
            }
        },
        _ => {}
    }  
}

fn set_direction(buffer: &mut CommandBuffer, world: &mut SubWorld, index: PlayerIndex, direction: Direction) {
    buffer.add_component(get_bat(world, index), direction);

}

fn remove_direction(buffer: &mut CommandBuffer, world: &mut SubWorld, index: PlayerIndex) {
    buffer.remove_component::<Direction>(get_bat(world, index));
}

fn get_bat(world: &mut SubWorld, index: PlayerIndex) -> Entity {
    <(Entity, &Bat)>::query()
        .iter_mut(world)
        .filter(|(_, bat)| ***bat == index)
        .map(|(entity, _)| *entity)
        .nth(0)
        .expect("No bat found")
}