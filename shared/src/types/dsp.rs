/// Identificador de la Voz (0 a N-1).
pub type VoiceId = u8;

/// Identificador de la Página de Parámetros (0 a 7, típicamente).
pub type PageId = u8;

/// Identificador del Knob dentro de la página (0 a 7).
pub type KnobId = u8;

/// Identificador del Motor de Síntesis (0=Basic, 1=FM, etc.).
pub type EngineId = u8;

/// Valor normalizado para parámetros (0.0 a 1.0).
/// El DSP es responsable de escalar esto al rango real (ej: 20Hz - 20kHz).
pub type NormalizedValue = f32;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TriggerMode {
    #[default]
    Retrigger, // Comportamiento agresivo: Siempre reinicia el ataque (Reset)
    Legato, // Comportamiento suave: Si ya suena, desliza (Sin Reset)
}

/// Valor constante para indicar que un comando afecta a TODAS las voces
/// o a un parámetro global del motor (ej: Reverb Mix).
pub const VOICE_GLOBAL: VoiceId = 255;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AudioCommand {
    // ------------------------------------------------------------------------
    // 1. GESTIÓN DE NOTAS (Lifecycle)
    // ------------------------------------------------------------------------
    /// Inicia una nota en una voz específica.
    /// NOTA: El Control ya ha hecho el cálculo de Nota MIDI -> Frecuencia Hz.
    NoteOn {
        voice_index: VoiceId,
        /// Frecuencia exacta en Hertz (ej: 440.0).
        frequency: f32,
        /// Ganancia inicial (0.0 a 1.0) derivada de la Velocity MIDI.
        gain: f32,
    },

    /// Inicia la fase de Release de la envolvente.
    NoteOff {
        voice_index: VoiceId,
    },

    /// Corte drástico (Hard Reset).
    /// Se usa cuando el Allocator roba una voz. El DSP debe silenciarla
    /// en el acto (0 ms release) y resetear osciladores/filtros para la nueva nota.
    KillVoice {
        voice_index: VoiceId,
    },

    // ------------------------------------------------------------------------
    // 2. PARÁMETROS AGNÓSTICOS (Patrón Macros/Páginas)
    // ------------------------------------------------------------------------
    /// El comando universal para modificar el sonido.
    /// Mapea coordenadas físicas (Página/Knob) a lógica de audio.
    SetMacro {
        /// Si es una voz específica (0-7), afecta solo a esa nota (ej: Poly Aftertouch).
        /// Si es VOICE_GLOBAL (255), afecta al estado global del motor para futuras notas
        /// y actualiza las voces activas según la lógica del motor.
        voice_index: VoiceId,

        page: PageId,
        knob: KnobId,

        /// SIEMPRE 0.0 a 1.0. El Control no sabe de Hz o ms.
        value: NormalizedValue,
    },

    // ------------------------------------------------------------------------
    // 3. SISTEMA Y MOTOR
    // ------------------------------------------------------------------------
    /// Cambia el algoritmo de síntesis completo en caliente.
    /// Al recibir esto, el DSP debería silenciar todo brevemente,
    /// cambiar el puntero del motor interno y limpiar buffers.
    LoadEngine {
        engine_id: EngineId,
    },

    /// Control de ganancia maestra del sistema (post-mezcla).
    /// Útil para evitar clipping final o control de volumen general.
    SetMasterVolume {
        value: NormalizedValue, // 0.0 a 1.0
    },

    /// Pánico. Apaga todo inmediatamente.
    StopAll,

    SetTriggerMode {
        mode: TriggerMode,
    },

    #[default]
    NoOp,
}
