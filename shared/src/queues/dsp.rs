use crate::types::dsp::AudioCommand;
use thingbuf::mpsc::StaticChannel;

pub const DSP_QUEUE_SIZE: usize = 65_536;

pub static DSP_CHANNEL: StaticChannel<AudioCommand, DSP_QUEUE_SIZE> = StaticChannel::new();
