use crate::domain::synthesizer::Synthesizer;
use shared::types::dsp::AudioCommand;

pub struct DspEngine {
    synthesizer: Synthesizer,
    // master_effects: Reverb,
    // global_lfo: Lfo,
}

impl DspEngine {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            synthesizer: Synthesizer::new(sample_rate),
        }
    }

    pub fn process_command(&mut self, command: AudioCommand) {
        // interceptar comandos globales (ej: "Panic", "MasterVolume")
        // antes de pasarlos al sinte.
        self.synthesizer.handle_command(command);
    }

    /// Genera el siguiente frame de audio (Audio Rate)
    pub fn next_stereo_frame(&mut self) -> (f32, f32) {
        // aplicar efectos post-sinte (Reverb, Delay)
        self.synthesizer.next_sample()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::types::dsp::AudioCommand;

    #[test]
    fn test_engine_produces_silence_initially() {
        let mut engine = DspEngine::new(44100.0);
        let (l, _) = engine.next_stereo_frame();
        assert!(l.abs() < 0.0001);
    }

    #[test]
    fn test_note_on_produces_sound() {
        let mut engine = DspEngine::new(44100.0);

        engine.process_command(AudioCommand::NoteOn {
            voice_index: 0,
            frequency: 440.0,
            gain: 1.0,
        });

        for _ in 0..100 {
            engine.next_stereo_frame();
        }

        let (l, _r) = engine.next_stereo_frame();
        assert!(l.abs() > 0.0);
    }
}
