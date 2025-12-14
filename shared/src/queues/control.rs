use thingbuf::mpsc::StaticChannel;

use crate::types::control::ControlEvent;

pub const QUEUE_SIZE: usize = 16_384;

// 3. Instanciamos el canal ESTÁTICO.
// Esto reserva la memoria en tiempo de compilación (.bss).
// StaticChannel<T, N>: Tipo, Tamaño.
pub static CONTROL_CHANNEL: StaticChannel<ControlEvent, QUEUE_SIZE> = StaticChannel::new();
