use crate::pages::amp::AmpPage;
use crate::pages::env::EnvAttrPage;
use crate::pages::filter::FilterPage;
use crate::pages::fx::FxPage;
use crate::pages::lfo::LfoPage;
use crate::pages::synth::SynthPage;
use crate::types::track::TrackId;

#[derive(Debug, Clone)]
pub struct Track {
    pub id: TrackId,
    pub synth: SynthPage,
    pub filter: FilterPage,
    pub amp: AmpPage,
    pub lfo: LfoPage,
    pub fx: FxPage,
    pub env_attr: EnvAttrPage,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            id: TrackId::Track1,
            synth: SynthPage::default(),
            filter: FilterPage::default(),
            amp: AmpPage::default(),
            lfo: LfoPage::default(),
            fx: FxPage::default(),
            env_attr: EnvAttrPage::default(),
        }
    }
}

impl Track {
    pub fn new(id: TrackId) -> Self {
        Self {
            id,
            synth: SynthPage::default(),
            filter: FilterPage::default(),
            amp: AmpPage::default(),
            lfo: LfoPage::default(),
            fx: FxPage::default(),
            env_attr: EnvAttrPage::default(),
        }
    }
}
