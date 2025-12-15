use crate::pages::schema::Parameter;
use crate::types::track::FilterType;

#[derive(Debug, Clone)]
pub struct FilterPage {
    pub k1: Parameter,
    pub k2: Parameter,
    pub k3: Parameter,
    pub k4: Parameter,
    pub k5: Parameter,
    pub k6: Parameter,
    pub k7: Parameter,
    pub k8: Parameter,
}

impl Default for FilterPage {
    fn default() -> Self {
        Self {
            k1: Parameter::float("Cutoff", 20000.0, 20.0, 20000.0, "Hz"),
            k2: Parameter::float("Res", 0.0, 0.0, 1.0, ""),
            k3: Parameter::float("EnvAmt", 0.0, -1.0, 1.0, ""),
            k4: Parameter::float("Drive", 0.0, 0.0, 1.0, ""),

            k5: Parameter::filter_type("Type", FilterType::LowPass24),
            k6: Parameter::float("KeyTrk", 0.5, 0.0, 1.0, ""),
            k7: Parameter::float("PreHPF", 10.0, 10.0, 500.0, "Hz"),
            k8: Parameter::float("Width", 0.0, 0.0, 1.0, ""),
        }
    }
}
