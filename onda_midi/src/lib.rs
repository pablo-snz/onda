use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use midir::{Ignore, MidiInput, MidiInputConnection};
use shared::types::control::{ControlEvent, MidiEvent};
use thingbuf::mpsc::StaticSender;
use thingbuf::recycling::DefaultRecycle;

struct ActiveConnection {
    port_name: String,
    _conn: MidiInputConnection<()>,
}

pub struct MidiHost {
    sender: StaticSender<ControlEvent, DefaultRecycle>,
    connections: Arc<Mutex<Vec<ActiveConnection>>>,
}

impl MidiHost {
    pub fn new(sender: StaticSender<ControlEvent, DefaultRecycle>) -> Self {
        Self {
            sender,
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(self) {
        println!("🎹 MIDI: Host iniciado (Hot-plug soportado).");

        let connections_state = self.connections.clone();
        let sender_template = self.sender.clone();

        thread::spawn(move || {
            loop {
                // 1. ESCANEO
                let current_port_names = match MidiInput::new("onda-scanner") {
                    Ok(mut temp_midi) => {
                        temp_midi.ignore(Ignore::None);
                        let ports = temp_midi.ports();
                        ports
                            .iter()
                            .map(|p| {
                                temp_midi
                                    .port_name(p)
                                    .unwrap_or_else(|_| "Unknown".to_string())
                            })
                            .collect::<Vec<String>>()
                    }
                    Err(e) => {
                        println!("⚠️ MIDI: Error creando scanner: {}", e);
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                };

                // Bloqueamos para modificar el estado
                if let Ok(mut active_conns) = connections_state.lock() {
                    // 2. DESCONEXIÓN
                    // Borramos las conexiones cuyo nombre ya no existe físicamente
                    active_conns.retain(|conn| {
                        let sigue_conectado = current_port_names.contains(&conn.port_name);
                        if !sigue_conectado {
                            println!("\x1b[31m🔌 MIDI: Desconectado: '{}'\x1b[0m", conn.port_name);
                        }
                        sigue_conectado
                    });

                    // 3. CONEXIÓN
                    for name in &current_port_names {
                        // Si no lo tenemos ya conectado...
                        if !active_conns.iter().any(|c| c.port_name == *name) {
                            println!(
                                "\x1b[33m🔌 MIDI: Detectado '{}', intentando conectar...\x1b[0m",
                                name
                            );

                            let tx = sender_template.clone();
                            let target_name = name.clone();

                            // Creamos el cliente DEDICADO para esta conexión
                            match MidiInput::new("onda-input") {
                                Ok(mut conn_client) => {
                                    conn_client.ignore(Ignore::None);

                                    // El cliente de conexión debe encontrar SU PROPIO puerto
                                    let all_ports = conn_client.ports();
                                    let port_to_connect = all_ports.iter().find(|p| {
                                        conn_client.port_name(p).unwrap_or_default() == target_name
                                    });

                                    if let Some(port) = port_to_connect {
                                        let conn_res = conn_client.connect(
                                            port,
                                            "onda-in",
                                            move |_, message, _| {
                                                process_midi_message(message, &tx);
                                            },
                                            (),
                                        );

                                        match conn_res {
                                            Ok(conn_obj) => {
                                                println!(
                                                    "\x1b[32m✅ MIDI: Conectado exitosamente a '{}'\x1b[0m",
                                                    target_name
                                                );
                                                active_conns.push(ActiveConnection {
                                                    port_name: target_name,
                                                    _conn: conn_obj,
                                                });
                                            }
                                            Err(e) => println!(
                                                "❌ MIDI: Error al conectar port '{}': {}",
                                                target_name, e
                                            ),
                                        }
                                    } else {
                                        println!(
                                            "⚠️ MIDI: El puerto '{}' desapareció antes de poder conectar.",
                                            target_name
                                        );
                                    }
                                }
                                Err(e) => println!(
                                    "❌ MIDI: No se pudo crear cliente para '{}': {}",
                                    target_name, e
                                ),
                            }
                        }
                    }
                }

                // Polling cada segundo para no saturar
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}

fn process_midi_message(bytes: &[u8], sender: &StaticSender<ControlEvent, DefaultRecycle>) {
    if bytes.is_empty() {
        return;
    }

    let status = bytes[0];
    let msg_type = status & 0xF0;

    match msg_type {
        // Note On
        0x90 => {
            let note = bytes[1];
            let velocity = bytes[2];
            // Standard MIDI: NoteOn con vel 0 es NoteOff
            let evt = if velocity > 0 {
                MidiEvent::NoteOn {
                    channel: 0,
                    note,
                    velocity,
                }
            } else {
                MidiEvent::NoteOff {
                    channel: 0,
                    note,
                    velocity: 0,
                }
            };
            let _ = sender.try_send(ControlEvent::Midi(evt));
        }
        // Note Off
        0x80 => {
            let note = bytes[1];
            let velocity = bytes[2];
            let _ = sender.try_send(ControlEvent::Midi(MidiEvent::NoteOff {
                channel: 0,
                note,
                velocity,
            }));
        }
        _ => {}
    }
}
