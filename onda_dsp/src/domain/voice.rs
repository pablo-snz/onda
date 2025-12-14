use ::shared::types::dsp::TriggerMode;
use fundsp::audiounit::AudioUnit;
use fundsp::hacker::*;

pub struct Voice {
    graph: Box<dyn AudioUnit>,
    pitch: Shared,
    gate: Shared,

    pub note: u8,
    trigger_mode: TriggerMode,
}

impl Voice {
    pub fn new(sample_rate: f64) -> Self {
        let pitch = shared(440.0);
        let gate = shared(0.0);

        // GRAFO DSP
        // 1. ADSR: Attack suave (10ms) para evitar clicks
        let envelope = var(&gate) >> adsr_live(0.01, 0.2, 0.8, 0.1);

        // 2. Oscilador + Filtro suave
        let osc = var(&pitch) >> saw() >> lowpole_hz(1500.0);

        // 3. Mezcla (Ganancia bajita para dejar espacio a la suma de voces)
        let graph = osc * envelope * 0.1;

        let mut boxed_graph = Box::new(graph) as Box<dyn AudioUnit>;
        boxed_graph.set_sample_rate(sample_rate);

        Self {
            graph: boxed_graph,
            pitch,
            gate,
            note: 0,
            // Inicializamos en modo percusivo por seguridad
            trigger_mode: TriggerMode::Retrigger,
        }
    }

    pub fn set_trigger_mode(&mut self, mode: TriggerMode) {
        self.trigger_mode = mode;
    }

    pub fn note_on(&mut self, note: u8, freq: f32, _gain: f32) {
        self.note = note;

        match self.trigger_mode {
            TriggerMode::Retrigger => {
                // MODO CLÁSICO (Percusivo)
                self.pitch.set_value(freq);
                // Reset obligatorio: Limpia filtros y pone ADSR a 0.
                self.graph.reset();
                self.gate.set_value(1.0);
            }
            TriggerMode::Legato => {
                // MODO LEGATO (Suave)
                let current_gate = self.gate.value();

                if current_gate > 0.0 {
                    // Ya estaba sonando: SOLO cambiamos frecuencia.
                    // Al no tocar el gate (ni reset), el ADSR no re-dispara.
                    // Obtenemos un slide perfecto.
                    self.pitch.set_value(freq);
                } else {
                    self.pitch.set_value(freq);
                    self.graph.reset();
                    self.gate.set_value(1.0);
                }
            }
        }
    }

    pub fn note_off(&mut self) {
        self.gate.set_value(0.0);
    }

    pub fn kill(&mut self) {
        self.gate.set_value(0.0);
        self.graph.reset();
    }

    pub fn next_sample(&mut self) -> f32 {
        self.graph.get_mono()
    }
}
