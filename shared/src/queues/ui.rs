use crate::pages::track::Track;
use thingbuf::mpsc::StaticChannel;

pub const UI_QUEUE_SIZE: usize = 16_384;

pub static UI_CHANNEL: StaticChannel<Track, UI_QUEUE_SIZE> = StaticChannel::new();
