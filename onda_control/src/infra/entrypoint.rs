use crate::application::ControlEngine;
use crate::domain::producer::Producer;
use crate::infra::static_producer::StaticProducer;
use shared::{
    pages::track::Track,
    types::{control::ControlEvent, dsp::AudioCommand},
};
use thingbuf::mpsc::{StaticReceiver, StaticSender};

pub struct ControlEntrypoint<D, U>
where
    D: Producer<AudioCommand>,
    U: Producer<Track>,
{
    engine: ControlEngine<D, U>,
    rx_control: StaticReceiver<ControlEvent>,
}

impl ControlEntrypoint<StaticProducer<AudioCommand>, StaticProducer<Track>> {
    pub fn new(
        tx_dsp: StaticSender<AudioCommand>,
        rx_control: StaticReceiver<ControlEvent>,
        tx_ui: StaticSender<Track>,
    ) -> Self {
        let domain_producer = StaticProducer::new(tx_dsp);
        let ui_producer = StaticProducer::new(tx_ui);
        let engine = ControlEngine::new(domain_producer, ui_producer);

        ControlEntrypoint { engine, rx_control }
    }

    pub fn start(&mut self) {
        loop {
            match self.rx_control.try_recv() {
                Ok(event) => match event {
                    ControlEvent::Midi(m) => self.engine.handle_midi_event(m),
                    ControlEvent::Ui(u) => {
                        let _ = u;
                    }
                    ControlEvent::NoOp => {}
                },
                Err(_empty) => {
                    core::hint::spin_loop();
                }
            }
        }
    }
}
