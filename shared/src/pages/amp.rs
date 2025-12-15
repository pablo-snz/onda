use crate::pages::schema::Parameter;

#[derive(Debug, Clone)]
pub struct AmpPage {
    // Env1 (Amp)
    pub k1: Parameter,
    pub k2: Parameter,
    pub k3: Parameter,
    pub k4: Parameter,
    // Env2 (Filter) - Shift
    pub k5: Parameter,
    pub k6: Parameter,
    pub k7: Parameter,
    pub k8: Parameter,
}

impl Default for AmpPage {
    fn default() -> Self {
        Self {
            k1: Parameter::float("A.Att", 5.0, 0.0, 5000.0, "ms"),
            k2: Parameter::float("A.Dec", 200.0, 0.0, 5000.0, "ms"),
            k3: Parameter::float("A.Sus", 0.8, 0.0, 1.0, ""),
            k4: Parameter::float("A.Rel", 500.0, 0.0, 5000.0, "ms"),

            k5: Parameter::float("F.Att", 0.0, 0.0, 5000.0, "ms"),
            k6: Parameter::float("F.Dec", 300.0, 0.0, 5000.0, "ms"),
            k7: Parameter::float("F.Sus", 0.0, 0.0, 1.0, ""),
            k8: Parameter::float("F.Rel", 200.0, 0.0, 5000.0, "ms"),
        }
    }
}
