use crate::infra::audio_output::AudioOutput;
use shared::types::dsp::AudioCommand;
use thingbuf::mpsc::StaticReceiver;
use thingbuf::recycling::DefaultRecycle;

pub struct DspEntrypoint {
    rx_dsp: Option<StaticReceiver<AudioCommand, DefaultRecycle>>,
    _stream: Option<cpal::Stream>,
}

impl DspEntrypoint {
    pub fn new(rx_queue: StaticReceiver<AudioCommand, DefaultRecycle>) -> Self {
        Self {
            rx_dsp: Some(rx_queue),
            _stream: None,
        }
    }

    pub fn start(&mut self) {
        println!("🔊 DSP: Iniciando sistema de audio...");

        if let Some(queue) = self.rx_dsp.take() {
            match AudioOutput::run_stream(queue) {
                Ok(stream) => {
                    println!("✅ DSP: Stream corriendo!");
                    // IMPORTANTE: Guardar el stream en self.
                    // Si _stream cae fuera de ámbito, el audio se corta.
                    self._stream = Some(stream);

                    // El hilo principal del DSP ahora se queda "aparcado"
                    // manteniendo el objeto DspEntrypoint vivo.
                    loop {
                        std::thread::park();
                    }
                }
                Err(e) => {
                    eprintln!("🔥 DSP FATAL ERROR: {}", e);
                }
            }
        }
    }
}
