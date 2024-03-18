use crate::{util::number2vlv, IntoMidiData, MidiData};

#[derive(Clone, Copy, Debug)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8, velocity: u8 },
    NoteAfterTouch { note: u8, amount: u8 },
    ProgramChange { code: u8 },
}

impl MidiEvent {
    pub fn size(&self) -> u16 {
        match self {
            Self::NoteOn { .. } => 3,
            Self::NoteOff { .. } => 3,
            Self::NoteAfterTouch { .. } => 3,
            Self::ProgramChange { .. } => 2,
        }
    }

    pub fn kind(&self) -> u8 {
        match self {
            Self::NoteOn { .. } => 0x90,
            Self::NoteOff { .. } => 0x80,
            Self::NoteAfterTouch { .. } => 0xA0,
            Self::ProgramChange { .. } => 0xc0,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::NoteOn { note, velocity } => vec![self.kind(), note, velocity],
            Self::NoteOff { note, velocity } => vec![self.kind(), note, velocity],
            Self::NoteAfterTouch { note, amount } => vec![self.kind(), note, amount],
            Self::ProgramChange { code } => vec![self.kind(), code],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MidiEventDelta {
    delta_time: u16,
    event: MidiEvent,
}

impl MidiEventDelta {
    pub fn new(delta_time: u16, event: MidiEvent) -> Self {
        Self { delta_time, event }
    }

    pub fn size(&self) -> u16 {
        self.event.size() + number2vlv(self.delta_time).len() as u16
    }
}

impl IntoMidiData for MidiEventDelta {
    fn into_midi_data(self) -> crate::MidiData {
        let mut data = MidiData::new();
        data.push_bytes(number2vlv(self.delta_time));
        data.push_bytes(self.event.into_bytes());
        data
    }
}
