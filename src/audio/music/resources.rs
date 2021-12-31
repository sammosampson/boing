use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MusicResources {
    Theme
}

#[derive(Default)]
pub struct MusicSourceCache {
    inner: HashMap<MusicResources, RawSoundSource>
}

impl MusicSourceCache {
    fn insert(&mut self, resource: MusicResources, data: &[u8]) {
        self.inner.insert(resource, RawSoundSource { bytes: data.to_vec() });
    }

    pub fn get(&self, resource: &MusicResources) -> Option<&RawSoundSource> {
        self.inner.get(resource)
    }
}

pub fn initialise_music_cache(sounds: &mut MusicSourceCache) {
    sounds.insert(MusicResources::Theme, &include_bytes!("../../../music/theme.ogg")[..]);
}