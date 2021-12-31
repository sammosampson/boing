use rodio::{
    OutputStream,
    OutputStreamHandle,
    Decoder,
    Sink,
    Source
};

use std::sync::mpsc::{
    Receiver,
    Sender,
    channel
};

use crate::prelude::*;

pub fn create_audio() -> AudioPlayer {
    AudioPlayer::new()
}

pub enum AudioEvent {
    PlaySound{ sound_id: usize, source: RawSoundSource, looped: bool, volume: f32 },
    StopSound{ sound_id: usize }
}

#[derive(Debug)]
pub enum AudioError {
    ImpossibleToLoadSound,
    ErrorStoppingAllSounds
}

pub struct AudioPlayer {
    event_sender: Sender<AudioEvent>,
    sounds_cursor: usize,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (event_sender, receiver) = channel();

        std::thread::spawn(move || {
            let controller = AudioController::new(receiver);
            controller.receive_audio_events();
        });

        AudioPlayer {
            event_sender,
            sounds_cursor: 0,
        }
    }

    pub fn play_sound(&mut self, resources: &SoundSourceCache, resource: SoundResources, volume: f32) -> Result<usize, AudioError> {
        let source = resources.get(&resource).unwrap();
                        
        let sound_id = self.sounds_cursor;
        
        println!("playing sound {:?} with id: {:?}", resource, sound_id);
        
        if let Ok(()) = self.event_sender.send(
            AudioEvent::PlaySound {
                sound_id,
                source: source.clone(),
                looped: false,
                volume
            }
        ) {
            self.sounds_cursor += 1;
            return Ok(sound_id);
        }
        return Err(AudioError::ImpossibleToLoadSound);
    }

    pub fn play_music(&mut self, resources: &MusicSourceCache, resource: MusicResources, volume: f32) -> Result<usize, AudioError> {
        let source = resources.get(&resource).unwrap();
                        
        let sound_id = self.sounds_cursor;
        
        println!("playing sound {:?} with id: {:?}", resource, sound_id);
        
        if let Ok(()) = self.event_sender.send(
            AudioEvent::PlaySound {
                sound_id,
                source: source.clone(),
                looped: true,
                volume
            }
        ) {
            self.sounds_cursor += 1;
            return Ok(sound_id);
        }
        return Err(AudioError::ImpossibleToLoadSound);
    }

    pub fn stop(&mut self, sound_id: usize) -> Result<(), AudioError> {
        println!("stopping sound: {:?}", sound_id);
        self.event_sender.send(AudioEvent::StopSound { sound_id }).map_err(|_|AudioError::ErrorStoppingAllSounds)   
    } 
}

pub struct AudioController {
    receiver: Receiver<AudioEvent>,
}

impl AudioController {
    pub(crate) fn new(receiver: Receiver<AudioEvent>) -> Self {
        Self {
            receiver
        }
    }

    pub fn receive_audio_events(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sinks: HashMap<usize, Sink> = HashMap::new();
    
        loop {
            if let Ok(message) = self.receiver.try_recv() {
                match message {
                    AudioEvent::PlaySound { source, looped, volume, sound_id } => {
                        println!("receiving sound to play with id: {:?}", sound_id);
        
                        let sink = Sink::try_new(&stream_handle).unwrap();
                        let source = Decoder::new(Cursor::new(source)).unwrap();
                        
                        if looped {
                            sink.append(source.repeat_infinite());
                            sink.set_volume(volume);
                            sinks.insert(sound_id, sink);
                        } else {
                            sink.append(source);
                            sink.set_volume(volume);
                            sink.detach();
                        }
                    }
                    AudioEvent::StopSound { sound_id } => {
                        if let Some(sink) = sinks.remove(&sound_id) {
                            sink.stop();
                            drop(sink);
                        }
                    }
                }
            }
        }
    }
}

