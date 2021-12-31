use crate::prelude::*;

#[derive(Debug)]
pub enum ApplicationError {
    RendererError(RendererError),
    TextureError(TextureError),
}
pub struct Application {
    world: World, 
    resources: Resources,
    start_schedule: Schedule,
    one_player_play_schedule: Schedule,
    two_player_play_schedule: Schedule,
    one_player_score_schedule: Schedule,
    two_player_score_schedule: Schedule,
    finish_schedule: Schedule,
    event_loop: SystemEventLoop
}

impl Application {
    pub fn build() -> Result<Self, ApplicationError> {
        let event_loop = create_system_event_loop();
        let world = build_world();
        let resources = build_resources(&event_loop)?;
        let start_schedule = build_start_schedule();
        let one_player_play_schedule = build_one_player_play_schedule();
        let two_player_play_schedule = build_two_player_play_schedule();
        let one_player_score_schedule = build_one_player_score_schedule();
        let two_player_score_schedule = build_two_player_score_schedule();
        let finish_schedule = build_finish_schedule();
       
        let application = Self {
            world,
            resources, 
            start_schedule,
            one_player_play_schedule,
            two_player_play_schedule,
            one_player_score_schedule,
            two_player_score_schedule,
            finish_schedule,
            event_loop
        };

        Ok(application)
    }

    pub fn run(&mut self) {
        loop {
            if !self.run_loop() {
                return;
            }
        }
    }
    
    fn run_loop(&mut self) -> bool {
        self.process_events();
        self.execute_schedule()     
    }

    fn process_events(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<SystemEventProducer>().unwrap();
        let mut event_channel = &mut self.resources.get_mut::<SystemEventChannel>().unwrap();
        self.event_loop.run(&mut event_producer, &mut event_channel);
    
    }

    fn execute_schedule(&mut self) -> bool {
        let current_state = self.resources.get::<GameState>().unwrap().status();
        let current_game_style = *self.resources.get::<GameStyle>().unwrap();
        
        match current_state {
            GameStatus::None => {},
            GameStatus::Starting => self.start_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Playing => {
                match current_game_style {
                    GameStyle::OnePlayer => self.one_player_play_schedule.execute(&mut self.world, &mut self.resources),
                    GameStyle::TwoPlayer => self.two_player_play_schedule.execute(&mut self.world, &mut self.resources),
                }
            },
            GameStatus::Scoring(_) =>  {
                match current_game_style {
                    GameStyle::OnePlayer => self.one_player_score_schedule.execute(&mut self.world, &mut self.resources),
                    GameStyle::TwoPlayer => self.two_player_score_schedule.execute(&mut self.world, &mut self.resources),
                }
            },
            GameStatus::Finishing => self.finish_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Exiting => return false
        };

        true
    }
}

fn build_resources(event_loop: &SystemEventLoop) -> Result<Resources, ApplicationError> {
    let game_style = create_game_style();
    let screen_renderer = create_screen_renderer(event_loop)?;
    let texture_cache = create_texture_cache(&screen_renderer)?;
    let item_renderer = create_item_renderer();
    let player_score = create_player_score();
    let game_timer = create_game_timer();
    let game_state = create_game_state();
    let system_event_producer = create_system_event_producer();
    let system_event_channel = create_system_event_channel();
    let audio = create_audio();
    let sound_cache = create_sound_cache();
    let music_cache = create_music_cache();
        
    let mut resources = Resources::default();
    &mut resources.insert(game_style);
    &mut resources.insert(screen_renderer);
    &mut resources.insert(texture_cache);
    &mut resources.insert(item_renderer);
    &mut resources.insert(game_timer);
    &mut resources.insert(player_score);
    &mut resources.insert(system_event_producer);
    &mut resources.insert(system_event_channel);
    &mut resources.insert(sound_cache);
    &mut resources.insert(music_cache);
    &mut resources.insert(audio);
    &mut resources.insert(game_state);
    Ok(resources)
}

pub fn create_texture_cache(screen_renderer: &ScreenRenderer) -> Result<TextureCache, ApplicationError> {
    let mut textures = TextureCache::default();

    initialise_texture_cache(&mut textures, screen_renderer)
        .map_err(|error| ApplicationError::TextureError(error))?;

    Ok(textures)
}


fn create_screen_renderer(event_loop: &SystemEventLoop) -> Result<ScreenRenderer, ApplicationError> {
    Ok(
        ScreenRenderer::new(&event_loop.get_loop())
            .map_err(|error| ApplicationError::RendererError(error))?
    )
}

fn create_sound_cache() -> SoundSourceCache {
    let mut sounds = SoundSourceCache::default();
    initialise_sound_cache(&mut sounds);
    sounds
}

fn create_music_cache() -> MusicSourceCache {
    let mut music = MusicSourceCache::default();
    initialise_music_cache(&mut music);
    music
}