use std::thread;

use onda_control::infra::entrypoint::ControlEntrypoint;
use onda_dsp::infra::entrypoint::DspEntrypoint;
use onda_midi::MidiHost;
use onda_ui::infra::keyboard_entrypoint::KeyboardEntrypoint;

use shared::queues::control::CONTROL_CHANNEL;
use shared::queues::dsp::DSP_CHANNEL;
use shared::queues::ui::UI_CHANNEL;

fn main() {
    println!("=== SINTETIZADOR RUST ===");

    // 1. SETUP DE COLAS (INFRAESTRUCTURA)
    // -----------------------------------

    // Cola DSP (SPSC)
    let (tx_dsp, rx_dsp) = DSP_CHANNEL.split();

    // Cola Control (MPSC - Static)
    let (tx_control, rx_control) = CONTROL_CHANNEL.split();

    // TODO: Añadir una cola de CONTROL -> UI para actualizaciones de estado
    // En esta cola Control enviará un snapshot del track actual a la UI cada vez que
    // haya un cambio relevante (nota on/off, cambio de parámetro, etc)
    // La UI hará cosas como "pintar" el estado de los LFOs, Envolventes, etc.
    // No hay una realidad entre el estado de LFO al parametro X pero podemos pintar una
    // animación aproximada en la UI. Simplificamos mucho el proceso y evitamos una cola desde
    // DSP a UI.

    // Cola UI (SPSC)
    let (tx_ui, _rx_ui) = UI_CHANNEL.split();

    // 2. LANZAMIENTO DE HILOS
    // -----------------------

    // A. Hilo DSP (Consumidor de Audio)
    let dsp_thread = thread::spawn(move || {
        let mut dsp = DspEntrypoint::new(rx_dsp);
        dsp.start();
    });

    // B. Hilo CONTROL (Lógica de Negocio)
    let control_thread = thread::spawn(move || {
        let mut control = ControlEntrypoint::new(tx_dsp, rx_control, tx_ui);
        control.start();
    });
    // C. Hilo MIDI (Hardware)
    // Clonamos el sender (StaticSender es barato de clonar)
    let tx_midi = tx_control.clone();
    let midi_host = MidiHost::new(tx_midi);
    midi_host.start(); // Esto lanza su propio hilo escáner en background

    // D. Hilo UI (Input) - Puede correr en el hilo principal
    // Clonamos el sender por si hubiera más hilos de UI en el futuro,
    // aunque aquí podríamos moverlo directamente.
    let tx_ui = tx_control.clone();

    // bloquea el main thread hasta ESC
    let ui = KeyboardEntrypoint::new(tx_ui);
    ui.start();

    let _ = dsp_thread.join();
    let _ = control_thread.join();
}
