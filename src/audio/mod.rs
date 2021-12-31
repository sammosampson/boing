mod audio_player;
mod sound;
mod music;

pub use audio_player::*;
pub use sound::*;
pub use music::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawSoundSource {
    pub bytes: Vec<u8>,
}

impl AsRef<[u8]> for RawSoundSource {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}