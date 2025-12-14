use shared::types::dsp::AudioCommand;

#[derive(Debug, PartialEq)]
pub enum ProducerError {
    QueueFull,
    Timeout,
    Disconnected,
}

pub trait Producer {
    /// Intenta enviar sin bloquear. Si está llena, devuelve error inmediatamente.
    fn try_send(&mut self, command: AudioCommand) -> Result<(), ProducerError>;

    /// Bloquea (spin-loop) hasta que haya hueco.
    fn block_send(&mut self, command: AudioCommand) -> Result<(), ProducerError>;
}
