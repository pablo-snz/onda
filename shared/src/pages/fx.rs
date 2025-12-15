use crate::pages::schema::Parameter;
use crate::types::track::FxType;

#[derive(Debug, Clone)]
pub struct FxPage {
    pub k1: Parameter,
    pub k2: Parameter,
    pub k3: Parameter,
    pub k4: Parameter,
    pub k5: Parameter,
    pub k6: Parameter,
    pub k7: Parameter,
    pub k8: Parameter,
}

impl Default for FxPage {
    fn default() -> Self {
        Self {
            k1: Parameter::fx_type("Algo", FxType::Delay),
            k2: Parameter::float("Mix", 0.0, 0.0, 1.0, ""),
            k3: Parameter::float("P.A", 0.5, 0.0, 1.0, ""),
            k4: Parameter::float("P.B", 0.5, 0.0, 1.0, ""),

            k5: Parameter::float("P.C", 0.5, 0.0, 1.0, ""),
            k6: Parameter::float("P.D", 0.5, 0.0, 1.0, ""),
            k7: Parameter::float("LoCut", 0.0, 0.0, 1000.0, "Hz"),
            k8: Parameter::float("HiCut", 20000.0, 1000.0, 20000.0, "Hz"),
        }
    }
}
