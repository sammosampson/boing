use crate::prelude::*;

#[system(simple)]
#[read_component(Bat)]
#[read_component(ScoreBoard)]
pub fn transition_state_to_playing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    #[resource] game_style: &GameStyle,
    #[resource] score: &mut PlayerScore,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {
    if game_state.has_entered() {
        return;
    }

    match game_state.previous_status() {
        GameStatus::Scoring(_) => {
            display_score(score, buffer, world);
            super::set_normal_bat_textures(buffer, world);
            add_ball(buffer);
        },
        GameStatus::Starting => {
            remove_menu_screen(buffer, world);
            set_player_game_style(buffer, world, *game_style);
            add_ball(buffer);
        },
        GameStatus::Finishing => {
            score.reset();
            display_score(score, buffer, world);
            remove_game_over_screen(buffer, world);
            add_ball(buffer);
        }
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

fn remove_menu_screen(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<MenuScreen>())
        .iter(world)
        .for_each(|entity| {
            remove_entity(buffer, *entity);
        });
}

fn set_player_game_style(buffer: &mut CommandBuffer, world: &SubWorld, game_style: GameStyle) {
    <(Entity, &Bat)>::query()
        .iter(world)
        .for_each(|(entity, bat)| {
            if **bat == PlayerIndex::Player2 && game_style == GameStyle::OnePlayer {
                buffer.remove_component::<Player>(*entity);
            }
        });
}

fn remove_game_over_screen(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<GameOverScreen>())
        .iter(world)
        .for_each(|entity| {
            remove_entity(buffer, *entity);
        });
}

fn display_score(
    player_score: &PlayerScore,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {
    <(Entity, &ScoreBoard)>::query()
        .iter(world)
        .for_each(|(entity, board)| {
            set_standard_score_texture(buffer, *entity, player_score.get(**board), **board);
        });
}
