use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};

use shared::types::dsp::AudioCommand;
use thingbuf::mpsc::StaticReceiver;
use thingbuf::recycling::DefaultRecycle;

use crate::application::dsp_engine::DspEngine;

pub struct AudioOutput;

impl AudioOutput {
    pub fn run_stream(queue: StaticReceiver<AudioCommand, DefaultRecycle>) -> Result<Stream> {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .context("❌ No se encontró dispositivo de salida de audio")?;

        println!(
            "🔊 Dispositivo de Audio: {}",
            device.name().unwrap_or_default()
        );

        let config: StreamConfig = device.default_output_config()?.into();
        let sample_rate = config.sample_rate.0 as f64;
        let channels = config.channels as usize;

        let mut engine = DspEngine::new(sample_rate);

        let queue_consumer = queue;

        println!("🎛️ Audio Engine Started at {:.1}Hz", sample_rate);

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                while let Ok(cmd) = queue_consumer.try_recv() {
                    engine.process_command(cmd);
                }

                for frame in data.chunks_mut(channels) {
                    let (left, right) = engine.next_stereo_frame();

                    if channels >= 2 {
                        frame[0] = left;
                        frame[1] = right;
                    } else {
                        frame[0] = (left + right) * 0.5;
                    }
                }
            },
            |err| eprintln!("🔥 Audio Error: {}", err),
            None,
        )?;

        stream.play()?;
        Ok(stream)
    }
}
