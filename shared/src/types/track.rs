#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TrackId {
    #[default]
    Track1,
    Track2,
    Track3,
    Track4,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SynthEngineType {
    #[default]
    Wavetable,
    Fm,
    Subtractive,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WavetableMachine {
    #[default]
    Basic,
    Harmonic,
    Vocal,
    Noise,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FmAlgorithm {
    #[default]
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FilterType {
    #[default]
    LowPass24,
    LowPass12,
    BandPass,
    HighPass,
    Notch,
    Comb,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EnvCurve {
    #[default]
    Exponential,
    Linear,
    Logarithmic,
    SCurve,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LfoShape {
    #[default]
    Triangle,
    Sine,
    Saw,
    Square,
    SampleAndHold,
    Glide,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FxType {
    #[default]
    Delay,
    Reverb,
    Chorus,
    Distortion,
    Bitcrusher,
}
