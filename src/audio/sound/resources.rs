use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SoundResources {
    Bounce(u8),
    BounceSynth,
    Hit(u8),
    HitSlow,
    HitMedium,
    HitFast,
    HitVeryFast,
    ScoreGoal,
    Up,
    Down
}

#[derive(Default)]
pub struct SoundSourceCache {
    inner: HashMap<SoundResources, RawSoundSource>
}

impl SoundSourceCache {
    fn insert(&mut self, resource: SoundResources, data: &[u8]) {
        self.inner.insert(resource, RawSoundSource { bytes: data.to_vec() });
    }

    pub fn get(&self, resource: &SoundResources) -> Option<&RawSoundSource> {
        self.inner.get(resource)
    }
}

pub fn initialise_sound_cache(sounds: &mut SoundSourceCache) {
    sounds.insert(SoundResources::Bounce(0), &include_bytes!("../../../sounds/bounce0.ogg")[..]);
    sounds.insert(SoundResources::Bounce(1), &include_bytes!("../../../sounds/bounce1.ogg")[..]);
    sounds.insert(SoundResources::Bounce(2), &include_bytes!("../../../sounds/bounce2.ogg")[..]);
    sounds.insert(SoundResources::Bounce(3), &include_bytes!("../../../sounds/bounce3.ogg")[..]);
    sounds.insert(SoundResources::Bounce(4), &include_bytes!("../../../sounds/bounce4.ogg")[..]);
    sounds.insert(SoundResources::BounceSynth, &include_bytes!("../../../sounds/bounce_synth0.ogg")[..]);
    sounds.insert(SoundResources::Hit(0), &include_bytes!("../../../sounds/hit0.ogg")[..]);
    sounds.insert(SoundResources::Hit(1), &include_bytes!("../../../sounds/hit1.ogg")[..]);
    sounds.insert(SoundResources::Hit(2), &include_bytes!("../../../sounds/hit2.ogg")[..]);
    sounds.insert(SoundResources::Hit(3), &include_bytes!("../../../sounds/hit3.ogg")[..]);
    sounds.insert(SoundResources::Hit(4), &include_bytes!("../../../sounds/hit4.ogg")[..]);
    sounds.insert(SoundResources::Hit(4), &include_bytes!("../../../sounds/hit4.ogg")[..]);
    sounds.insert(SoundResources::HitSlow, &include_bytes!("../../../sounds/hit_slow0.ogg")[..]);    
    sounds.insert(SoundResources::HitMedium, &include_bytes!("../../../sounds/hit_medium0.ogg")[..]);
    sounds.insert(SoundResources::HitFast, &include_bytes!("../../../sounds/hit_fast0.ogg")[..]);
    sounds.insert(SoundResources::HitVeryFast, &include_bytes!("../../../sounds/hit_veryfast0.ogg")[..]);
    sounds.insert(SoundResources::ScoreGoal, &include_bytes!("../../../sounds/score_goal0.ogg")[..]);
    sounds.insert(SoundResources::Up, &include_bytes!("../../../sounds/up.ogg")[..]);
    sounds.insert(SoundResources::Down, &include_bytes!("../../../sounds/down.ogg")[..]);
}