pub type Channel = u8; // 0..15
pub type Note = u8; // 0..127
pub type Velocity = u8; // 0..127
pub type Page = u8; // 0..15

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum ControlEvent {
    Midi(MidiEvent),
    Ui(UiEvent),
    #[default]
    NoOp,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MidiEvent {
    NoteOff {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    NoteOn {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum UiEvent {
    Knob {
        page: Page,
        knob: u8,
        value: u16, // 0..65535
    },
    Engine,
}
