use crate::domain::{producer::Producer, voice_allocator::VoiceAllocator};
use shared::{
    constants::voices::MAX_VOICES,
    pages::track::Track,
    types::{control::MidiEvent, dsp::AudioCommand},
};

fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

fn velocity_to_gain(velocity: u8) -> f32 {
    velocity as f32 / 127.0
}

pub struct ControlEngine<D, U> {
    state: VoiceAllocator<MAX_VOICES>,
    dsp_tx: D,
    #[allow(dead_code)] // TODO: Remove when UI messages are implemented
    ui_tx: U,
}

impl<D, U> ControlEngine<D, U>
where
    D: Producer<AudioCommand>,
    U: Producer<Track>,
{
    pub fn new(dsp_tx: D, ui_tx: U) -> Self {
        ControlEngine {
            state: VoiceAllocator::new(),
            dsp_tx,
            ui_tx,
        }
    }

    pub fn handle_midi_event(&mut self, event: MidiEvent) {
        match event {
            MidiEvent::NoteOn {
                channel,
                note,
                velocity,
            } => {
                if velocity == 0 {
                    if let Some(voice_id) = self.state.note_off(channel, note) {
                        self.dsp_tx
                            .block_send(AudioCommand::NoteOff {
                                voice_index: voice_id,
                            })
                            .unwrap();
                    }
                    return;
                }

                let alloc = self.state.note_on(channel, note, velocity);

                if let Some(st) = alloc.stolen {
                    println!("♻️ Stealing voice {}", st.voice_id);
                }

                self.dsp_tx
                    .block_send(AudioCommand::NoteOn {
                        voice_index: alloc.voice_id,
                        frequency: midi_to_hz(note),
                        gain: velocity_to_gain(velocity),
                    })
                    .unwrap();
            }

            MidiEvent::NoteOff { channel, note, .. } => {
                if let Some(voice_id) = self.state.note_off(channel, note) {
                    self.dsp_tx
                        .block_send(AudioCommand::NoteOff {
                            voice_index: voice_id,
                        })
                        .unwrap();
                }
            }
        }
    }
}
