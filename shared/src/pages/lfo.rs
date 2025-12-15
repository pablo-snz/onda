use crate::pages::schema::Parameter;
use crate::types::track::LfoShape;

#[derive(Debug, Clone)]
pub struct LfoPage {
    pub k1: Parameter,
    pub k2: Parameter,
    pub k3: Parameter,
    pub k4: Parameter,
    pub k5: Parameter,
    pub k6: Parameter,
    pub k7: Parameter,
    pub k8: Parameter,
}

impl Default for LfoPage {
    fn default() -> Self {
        Self {
            k1: Parameter::lfo_shape("Shape", LfoShape::Triangle),
            k2: Parameter::float("Rate", 1.0, 0.1, 50.0, "Hz"),
            k3: Parameter::float("Depth", 1.0, 0.0, 1.0, ""),
            k4: Parameter::bool("Sync", false),

            k5: Parameter::float("Phase", 0.0, 0.0, 360.0, "deg"),
            k6: Parameter::float("Fade", 0.0, 0.0, 5000.0, "ms"),
            k7: Parameter::float("Smooth", 0.0, 0.0, 1.0, ""),
            k8: Parameter::bool("Bipolar", true),
        }
    }
}
