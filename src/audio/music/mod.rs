mod resources;
pub use resources::*;
use crate::prelude::*;

pub fn create_music_components(resource: MusicResources) -> (Music, ) {
    (Music(resource), )
}

#[derive(Debug, Copy, Clone)]
pub struct Music(pub MusicResources);

impl Deref for Music {
    type Target = MusicResources;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
