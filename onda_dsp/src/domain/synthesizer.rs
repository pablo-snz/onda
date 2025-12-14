use crate::domain::voice::Voice;
use shared::{constants::voices::MAX_VOICES, types::dsp::AudioCommand};

pub struct Synthesizer {
    voices: Vec<Voice>,
    _sample_rate: f64,
}

impl Synthesizer {
    pub fn new(sample_rate: f64) -> Self {
        println!(
            "🎛️ POLY-SYNTH: Inicializando {} voces a {:.1}Hz",
            MAX_VOICES, sample_rate
        );

        let mut voices = Vec::with_capacity(MAX_VOICES);
        for _ in 0..MAX_VOICES {
            voices.push(Voice::new(sample_rate));
        }

        Self {
            voices,
            _sample_rate: sample_rate,
        }
    }

    pub fn handle_command(&mut self, command: AudioCommand) {
        match command {
            AudioCommand::NoteOn {
                voice_index,
                frequency,
                gain,
            } => {
                let idx = voice_index as usize;
                if idx < self.voices.len() {
                    self.voices[idx].note_on(0, frequency, gain);
                }
            }
            AudioCommand::NoteOff { voice_index } => {
                let idx = voice_index as usize;
                if idx < self.voices.len() {
                    self.voices[idx].note_off();
                }
            }
            AudioCommand::KillVoice { voice_index } => {
                let idx = voice_index as usize;
                if idx < self.voices.len() {
                    self.voices[idx].kill();
                }
            }
            AudioCommand::SetTriggerMode { mode } => {
                for voice in &mut self.voices {
                    voice.set_trigger_mode(mode);
                }
            }
            AudioCommand::NoOp => {}
            _ => {}
        }
    }

    pub fn next_sample(&mut self) -> (f32, f32) {
        let mut mix_left = 0.0;
        let mut mix_right = 0.0;

        for voice in &mut self.voices {
            let sample = voice.next_sample();
            mix_left += sample;
            mix_right += sample;
        }

        // Ganancia unitaria y saturación suave para evitar distorsión digital
        let master_gain = 4.0;

        mix_left *= master_gain;
        mix_right *= master_gain;

        mix_left = mix_left.tanh();
        mix_right = mix_right.tanh();

        (mix_left, mix_right)
    }
}
