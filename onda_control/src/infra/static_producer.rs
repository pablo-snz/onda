use crate::domain::producer::{Producer, ProducerError};
use shared::types::dsp::AudioCommand;
use thingbuf::mpsc::StaticSender;
use thingbuf::mpsc::errors::TrySendError;
use thingbuf::recycling::DefaultRecycle;

pub struct StaticProducer {
    sender: StaticSender<AudioCommand, DefaultRecycle>,
}

impl StaticProducer {
    pub fn new(sender: StaticSender<AudioCommand, DefaultRecycle>) -> Self {
        Self { sender }
    }
}

impl Producer for StaticProducer {
    fn try_send(&mut self, command: AudioCommand) -> Result<(), ProducerError> {
        match self.sender.try_send(command) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Err(ProducerError::QueueFull),
            Err(TrySendError::Closed(_)) => Err(ProducerError::Disconnected),
            Err(_) => Err(ProducerError::Disconnected),
        }
    }

    fn block_send(&mut self, mut command: AudioCommand) -> Result<(), ProducerError> {
        loop {
            match self.sender.try_send(command) {
                Ok(_) => return Ok(()),

                Err(TrySendError::Full(returned_cmd)) => {
                    command = returned_cmd;
                    core::hint::spin_loop();
                }

                Err(TrySendError::Closed(_)) => return Err(ProducerError::Disconnected),

                Err(_) => return Err(ProducerError::Disconnected),
            }
        }
    }
}
