mod rendering;
mod events;
mod state;
mod movement;
mod time;
mod collision;
mod animation;
mod effects;
mod audio;
mod world;
mod screens;

pub use legion::*;
pub use legion::query::Query;
pub use legion::systems::CommandBuffer;
pub use legion::world::SubWorld;

use legion::systems::ParallelRunnable;

pub fn build_world() -> World {
    let world = World::default();
    world
}

pub fn build_start_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_starting_system())
        .add_system(events::proliferate_system_events_system())
        .flush()
        .add_system(time::calculate_elapsed_time_system())
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(screens::menu_screen_input_system())
        .add_system(screens::set_menu_screen_texture_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
        .add_thread_local(effects::remove_dead_effects_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}

pub fn build_one_player_play_schedule() -> Schedule {
    build_play_schedule(movement::calculate_computer_player_two_direction_system())
}

pub fn build_two_player_play_schedule() -> Schedule {
    build_play_schedule(movement::set_player_two_direction_from_input_system())
}

fn build_play_schedule<T: 'static + ParallelRunnable>(player_two_control_system: T) -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_playing_system())
        .flush()
        .add_system(events::proliferate_system_events_system())
        .add_system(movement::initialise_movement_system())
        .flush()
        .add_system(time::calculate_elapsed_time_system())
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(movement::set_player_one_direction_from_input_system())
        .add_system(player_two_control_system)
        .flush()
        .add_system(movement::play_bat_movement_sounds_system())
        .add_system(movement::set_bat_heading_from_direction_system())
        .add_system(movement::reset_bat_heading_with_no_direction_system())
        .add_system(collision::contain_bat_in_bounds_system())
        .add_system(movement::set_velocity_given_heading_system())
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
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
        .add_thread_local(effects::remove_dead_effects_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}

pub fn build_one_player_score_schedule() -> Schedule {
    build_score_schedule(movement::calculate_computer_player_two_direction_system())
}

pub fn build_two_player_score_schedule() -> Schedule {
    build_score_schedule(movement::set_player_two_direction_from_input_system())
}

pub fn build_score_schedule<T: 'static + ParallelRunnable>(player_two_control_system: T) -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_scored_system())
        .add_system(events::proliferate_system_events_system())
        .flush()
        .add_system(time::calculate_elapsed_time_system())
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(movement::set_player_one_direction_from_input_system())
        .add_system(player_two_control_system)
        .flush()
        .add_system(movement::play_bat_movement_sounds_system())
        .add_system(movement::set_bat_heading_from_direction_system())
        .add_system(movement::reset_bat_heading_with_no_direction_system())
        .add_system(collision::contain_bat_in_bounds_system())
        .add_system(movement::set_velocity_given_heading_system())
        .add_system(movement::apply_velocity_to_position_system())
        .flush()
        .add_thread_local(movement::set_position_system())
        .flush()
        .add_thread_local(animation::render_first_animation_frame_system())
        .add_thread_local(animation::render_animation_frame_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
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
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
        .flush()
        .add_thread_local(effects::remove_dead_effects_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}
