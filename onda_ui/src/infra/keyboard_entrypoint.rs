use device_query::{DeviceQuery, DeviceState, Keycode};
use shared::types::control::{ControlEvent, MidiEvent};
use std::{thread, time::Duration};

use thingbuf::mpsc::StaticSender;

pub struct KeyboardEntrypoint {
    sender: StaticSender<ControlEvent>,
}

impl KeyboardEntrypoint {
    pub fn new(sender: StaticSender<ControlEvent>) -> Self {
        Self { sender }
    }

    pub fn start(&self) {
        println!("🎹 UI: Escuchando teclado (A, W, S, D... ESC para salir)");

        let device_state = DeviceState::new();
        let mut prev_keys: Vec<Keycode> = vec![];

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();

            // 1. Note ON
            for key in &keys {
                if !prev_keys.contains(key) {
                    if let Some(note) = key_to_midi(*key) {
                        let evt = MidiEvent::NoteOn {
                            channel: 0,
                            note,
                            velocity: 100,
                        };
                        // Usamos try_send (no bloqueante, síncrono)
                        let _ = self.sender.try_send(ControlEvent::Midi(evt));
                    } else if *key == Keycode::Escape {
                        std::process::exit(0);
                    }
                }
            }

            // 2. Note OFF
            for key in &prev_keys {
                // Si la tecla 'key' todavía está en la lista actual 'keys',
                // significa que sigue pulsada. No hacemos nada y pasamos a la siguiente.
                if keys.contains(key) {
                    continue;
                }

                // Si llegamos aquí, es que la tecla YA NO está pulsada (se ha soltado).
                // Ahora comprobamos si es una nota MIDI válida.
                if let Some(note) = key_to_midi(*key) {
                    let evt = MidiEvent::NoteOff {
                        channel: 0,
                        note,
                        velocity: 0,
                    };
                    let _ = self.sender.try_send(ControlEvent::Midi(evt));
                }
            }

            prev_keys = keys;
            thread::sleep(Duration::from_millis(10));
        }
    }
}

// Helper (igual que antes)
fn key_to_midi(key: Keycode) -> Option<u8> {
    match key {
        Keycode::A => Some(60), // C4
        Keycode::W => Some(61),
        Keycode::S => Some(62),
        Keycode::E => Some(63),
        Keycode::D => Some(64),
        Keycode::F => Some(65),
        Keycode::T => Some(66),
        Keycode::G => Some(67),
        Keycode::K => Some(72), // C5
        _ => None,
    }
}
