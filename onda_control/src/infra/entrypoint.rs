use crate::application::ControlEngine;
use crate::domain::producer::Producer as DomainProducer;
use crate::infra::static_producer::StaticProducer;
use shared::types::{control::ControlEvent, dsp::AudioCommand};

use thingbuf::mpsc::{StaticReceiver, StaticSender};

pub struct ControlEntrypoint<T: DomainProducer> {
    engine: ControlEngine<T>,
    rx_control: StaticReceiver<ControlEvent>,
}

impl ControlEntrypoint<StaticProducer> {
    pub fn new(
        tx_dsp: StaticSender<AudioCommand>,
        rx_control: StaticReceiver<ControlEvent>,
    ) -> Self {
        let domain_producer = StaticProducer::new(tx_dsp);
        let engine = ControlEngine::new(domain_producer);

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
