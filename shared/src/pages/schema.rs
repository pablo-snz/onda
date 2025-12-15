use crate::types::track::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageId {
    P1Synth,
    P2Filter,
    P3AmpEnv,
    P4Lfo,
    ShP1Fx,
    ShP2Voice,
    ShP3EnvAttr,
    ShP4ModMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnobId {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamValue {
    Empty,
    Float(f32),
    Bool(bool),
    Filter(FilterType),
    Curve(EnvCurve),
    Lfo(LfoShape),
    Fx(FxType),
    WavetableMachine(WavetableMachine),
    FmAlgorithm(FmAlgorithm),
}

#[derive(Debug, Clone, Copy)]
pub struct UiEvent {
    pub track: TrackId,
    pub page: PageId,
    pub knob: KnobId,
    pub value: ParamValue,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub label: &'static str,
    pub unit: &'static str,
    pub val: ParamValue,
    pub min: f32,
    pub max: f32,
}

impl Parameter {
    pub fn float(label: &'static str, val: f32, min: f32, max: f32, unit: &'static str) -> Self {
        Self {
            label,
            unit,
            val: ParamValue::Float(val),
            min,
            max,
        }
    }

    pub fn bool(label: &'static str, val: bool) -> Self {
        Self {
            label,
            unit: "",
            val: ParamValue::Bool(val),
            min: 0.0,
            max: 1.0,
        }
    }

    pub fn empty() -> Self {
        Self {
            label: "",
            unit: "",
            val: ParamValue::Empty,
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn wt_machine(label: &'static str, val: WavetableMachine) -> Self {
        Self {
            label,
            unit: "",
            val: ParamValue::WavetableMachine(val),
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn filter_type(label: &'static str, val: FilterType) -> Self {
        Self {
            label,
            unit: "",
            val: ParamValue::Filter(val),
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn curve(label: &'static str, val: EnvCurve) -> Self {
        Self {
            label,
            unit: "",
            val: ParamValue::Curve(val),
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn lfo_shape(label: &'static str, val: LfoShape) -> Self {
        Self {
            label,
            unit: "",
            val: ParamValue::Lfo(val),
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn fx_type(label: &'static str, val: FxType) -> Self {
        Self {
            label,
            unit: "",
            val: ParamValue::Fx(val),
            min: 0.0,
            max: 0.0,
        }
    }
}
