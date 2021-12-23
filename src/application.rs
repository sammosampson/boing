use crate::prelude::*;

#[derive(Debug)]
pub enum ApplicationError {
    RendererError(RendererError),
    SoundError(SoundError)
}
pub struct Application {
    pub world: World, 
    pub resources: Resources,
    play_schedule: Schedule,
    score_schedule: Schedule,
    event_loop: SystemEventLoop
}

impl Application {
    pub fn build() -> Result<Self, ApplicationError> {
        let event_loop = create_system_event_loop();
        let world = build_world();
        let resources = build_resources(&event_loop)?;
        let play_schedule = build_play_schedule();
        let score_schedule = build_score_schedule();
       
        let application = Self {
            world,
            resources, 
            play_schedule,
            score_schedule,
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
        match current_state {
            GameStatus::None => {},
            GameStatus::Playing => self.play_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Scoring(_) => self.score_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Exiting => return false
        };        
        true
    }
}

fn build_resources(event_loop: &SystemEventLoop) -> Result<Resources, ApplicationError> {
    let screen_renderer = create_screen_renderer(event_loop)?;
    let item_renderer = create_item_renderer();
    let game_timer = create_game_timer();
    let game_state = create_game_state();
    let system_event_producer = create_system_event_producer();
    let system_event_channel = create_system_event_channel();
    let audio_player = create_audio_player()?;
        
    let mut resources = Resources::default();
    &mut resources.insert(screen_renderer);
    &mut resources.insert(item_renderer);
    &mut resources.insert(game_timer);
    &mut resources.insert(system_event_producer);
    &mut resources.insert(system_event_channel);
    &mut resources.insert(audio_player);
    &mut resources.insert(game_state);
    Ok(resources)
}

fn create_screen_renderer(event_loop: &SystemEventLoop) -> Result<ScreenRenderer, ApplicationError> {
    Ok(
        ScreenRenderer::new(&event_loop.get_loop())
            .map_err(|error| ApplicationError::RendererError(error))?
    )
}

fn create_audio_player() -> Result<AudioPlayer, ApplicationError> {
    Ok(
        AudioPlayer::new().map_err(|error| ApplicationError::SoundError(error))?
    )
}