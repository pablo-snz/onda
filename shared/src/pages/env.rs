use crate::pages::schema::Parameter;
use crate::types::track::EnvCurve;

#[derive(Debug, Clone)]
pub struct EnvAttrPage {
    pub k1: Parameter,
    pub k2: Parameter,
    pub k3: Parameter,
    pub k4: Parameter,
    pub k5: Parameter,
    pub k6: Parameter,
    pub k7: Parameter,
    pub k8: Parameter,
}

impl Default for EnvAttrPage {
    fn default() -> Self {
        Self {
            k1: Parameter::curve("A.Curve", EnvCurve::Exponential),
            k2: Parameter::bool("A.Loop", false),
            k3: Parameter::bool("A.Retrig", true),
            k4: Parameter::float("A.Scale", 1.0, 0.1, 2.0, "x"),

            k5: Parameter::curve("F.Curve", EnvCurve::Exponential),
            k6: Parameter::bool("F.Loop", false),
            k7: Parameter::bool("F.Retrig", true),
            k8: Parameter::float("F.Scale", 1.0, 0.1, 2.0, "x"),
        }
    }
}
