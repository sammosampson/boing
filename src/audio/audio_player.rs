use rodio::{
    OutputStream,
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
    PlaySound{ source: RawSoundSource, looped: bool, volume: f32 }
}

#[derive(Debug)]
pub enum AudioError {
    LoadSoundError
}

pub struct AudioPlayer {
    event_sender: Sender<AudioEvent>
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (event_sender, receiver) = channel();

        std::thread::spawn(move || {
            let controller = AudioController::new(receiver);
            controller.receive_audio_events();
        });

        AudioPlayer { event_sender }
    }

    pub fn play_sound(&mut self, resources: &SoundSourceCache, resource: SoundResources, volume: f32) -> Result<(), AudioError> {
        let source = resources.get(&resource).unwrap();
        println!("playing sound {:?}", resource);
        self.play_audio(source, volume, false)
    }

    pub fn play_music(&mut self, resources: &MusicSourceCache, resource: MusicResources, volume: f32) -> Result<(), AudioError> {
        let source = resources.get(&resource).unwrap();
        println!("playing music {:?}", resource);
        self.play_audio(source, volume, true)
    }

    fn play_audio(&mut self, source: &RawSoundSource, volume: f32, looped: bool) -> Result<(), AudioError> {
        Ok(self.event_sender
           .send(AudioEvent::PlaySound { source: source.clone(), looped, volume })
           .map_err(|_| AudioError::LoadSoundError)?)
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
        
        loop {
            if let Ok(message) = self.receiver.try_recv() {
                match message {
                    AudioEvent::PlaySound { source, looped, volume, } => {
                        let sink = Sink::try_new(&stream_handle)
                            .expect("Cannot create audio sink");

                        let source = Decoder::new(Cursor::new(source))
                            .expect("Cannot decode audio");
                        
                        if looped {
                            sink.append(source.repeat_infinite());
                        } else {
                            sink.append(source);
                        }
                        sink.set_volume(volume);
                        sink.detach();
                    }                
                }
            }
        }
    }
}

