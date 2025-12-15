use crate::pages::schema::Parameter;
use crate::types::track::WavetableMachine;

#[derive(Debug, Clone)]
pub struct WavetablePage {
    pub k1: Parameter, // Machine
    pub k2: Parameter, // Tune
    pub k3: Parameter, // Fine
    pub k4: Parameter, // Position
    pub k5: Parameter, // Warp
    pub k6: Parameter, // Fold
    pub k7: Parameter, // Sub
    pub k8: Parameter, // Noise
}

impl Default for WavetablePage {
    fn default() -> Self {
        Self {
            k1: Parameter::wt_machine("Machine", WavetableMachine::Basic),
            k2: Parameter::float("Tune", 0.0, -24.0, 24.0, "st"),
            k3: Parameter::float("Fine", 0.0, -50.0, 50.0, "ct"),
            k4: Parameter::float("Pos", 0.0, 0.0, 1.0, "%"),

            k5: Parameter::float("Warp", 0.0, 0.0, 1.0, "%"),
            k6: Parameter::float("Fold", 0.0, 0.0, 1.0, "%"),
            k7: Parameter::float("Sub", 0.0, 0.0, 1.0, "%"),
            k8: Parameter::float("Noise", 0.0, 0.0, 1.0, "%"),
        }
    }
}
