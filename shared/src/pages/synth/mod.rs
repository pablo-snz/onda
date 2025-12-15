pub mod wavetable;

use self::wavetable::WavetablePage;

#[derive(Debug, Clone)]
pub enum SynthPage {
    Wavetable(WavetablePage),
    // Fm(FmPage),
}

impl Default for SynthPage {
    fn default() -> Self {
        SynthPage::Wavetable(WavetablePage::default())
    }
}
