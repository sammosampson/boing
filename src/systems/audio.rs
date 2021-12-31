use crate::prelude::*;

#[system(for_each)]
pub fn play_sound(
    entity: &Entity,
    sound: &Sound,
    #[resource] resources: &SoundSourceCache,
    #[resource] audio_player: &mut AudioPlayer,
    buffer: &mut CommandBuffer,
) {
    audio_player
        .play_sound(resources, **sound, 1.0)
        .expect("cannot play sound");
        
    buffer.remove(*entity);
}  

#[system(for_each)]
pub fn play_music(
    entity: &Entity,
    music: &Music,
    #[resource] resources: &MusicSourceCache,
    #[resource] audio_player: &mut AudioPlayer,
    buffer: &mut CommandBuffer,
) {
    audio_player
        .play_music(resources, **music, 0.25)
        .expect("cannot play music");
        
    buffer.remove(*entity);
}  
