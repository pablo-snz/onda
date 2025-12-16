use crate::domain::producer::{Producer, ProducerError};
use thingbuf::mpsc::StaticSender;
use thingbuf::mpsc::errors::TrySendError;
use thingbuf::recycling::DefaultRecycle;

pub struct StaticProducer<T>
where
    T: 'static,
{
    sender: StaticSender<T, DefaultRecycle>,
}

impl<T> StaticProducer<T> {
    pub fn new(sender: StaticSender<T, DefaultRecycle>) -> Self {
        Self { sender }
    }
}

impl<T> Producer<T> for StaticProducer<T>
where
    T: 'static + Send + Clone + Default,
{
    fn try_send(&mut self, item: T) -> Result<(), ProducerError> {
        match self.sender.try_send(item) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Err(ProducerError::QueueFull),
            Err(TrySendError::Closed(_)) => Err(ProducerError::Disconnected),
            Err(_) => Err(ProducerError::Disconnected),
        }
    }

    fn block_send(&mut self, mut item: T) -> Result<(), ProducerError> {
        loop {
            match self.sender.try_send(item) {
                Ok(_) => return Ok(()),
                Err(TrySendError::Full(returned_item)) => {
                    item = returned_item;
                    core::hint::spin_loop();
                }
                Err(TrySendError::Closed(_)) => return Err(ProducerError::Disconnected),
                Err(_) => return Err(ProducerError::Disconnected),
            }
        }
    }
}
