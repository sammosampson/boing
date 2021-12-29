mod rendering;
mod events;
mod state;
mod movement;
mod time;
mod collision;
mod animation;
mod effects;
mod sound;
mod world;
mod screens;

pub use legion::*;
pub use legion::query::Query;
pub use legion::systems::CommandBuffer;
pub use legion::world::SubWorld;

pub fn build_world() -> World {
    let world = World::default();
    world
}

pub fn build_start_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_starting_system())
        .add_system(events::proliferate_system_events_system())
        .flush()
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(screens::menu_screen_input_system())
        .add_system(screens::set_menu_screen_texture_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(sound::play_sound_system())
        .add_thread_local(effects::remove_dead_effects_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}

pub fn build_play_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_playing_system())
        .add_system(events::proliferate_system_events_system())
        .add_system(movement::initialise_movement_system())
        .flush()
        .add_system(time::calculate_elapsed_time_system())
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(movement::set_player_one_heading_from_input_system())
        .add_system(movement::set_player_two_heading_from_input_system())
        .add_system(collision::contain_bat_in_bounds_system())
        .add_system(movement::set_velocity_given_heading_system())
        .add_system(movement::set_bat_movement_sounds_system())
        .add_system(movement::apply_velocity_to_position_system())
        .add_system(collision::check_collision_system())
        .flush()
        .add_system(collision::handle_wall_collision_system())
        .add_system(collision::handle_bat_collision_system())
        .add_system(collision::handle_goal_collision_system())
        .flush()
        .add_thread_local(movement::set_position_system())
        .flush()
        .add_thread_local(animation::render_first_animation_frame_system())
        .add_thread_local(animation::render_animation_frame_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(sound::play_sound_system())
        .add_thread_local(effects::remove_dead_effects_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}

pub fn build_score_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_scored_system())
        .add_system(events::proliferate_system_events_system())
        .flush()
        .add_system(time::calculate_elapsed_time_system())
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(movement::set_player_one_heading_from_input_system())
        .add_system(movement::set_player_two_heading_from_input_system())
        .add_system(collision::contain_bat_in_bounds_system())
        .add_system(movement::set_velocity_given_heading_system())
        .add_system(movement::set_bat_movement_sounds_system())
        .add_system(movement::apply_velocity_to_position_system())
        .flush()
        .add_thread_local(movement::set_position_system())
        .flush()
        .add_thread_local(animation::render_first_animation_frame_system())
        .add_thread_local(animation::render_animation_frame_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(sound::play_sound_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}

pub fn build_finish_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_finishing_system())
        .add_system(events::proliferate_system_events_system())
        .flush()
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(screens::game_over_screen_input_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(sound::play_sound_system())
        .flush()
        .add_thread_local(effects::remove_dead_effects_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}
