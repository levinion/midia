use crate::{IntoMidiData, MidiData};

pub enum MidiEvent {
    NoteOn { note: u8, dynamics: u8 },
    NoteOff { note: u8, dynamics: u8 },
    ChangeInstrument { code: u8 },
}

impl MidiEvent {
    pub fn size(&self) -> u16 {
        match self {
            Self::NoteOn { .. } => 3,
            Self::NoteOff { .. } => 3,
            Self::ChangeInstrument { .. } => 2,
        }
    }

    pub fn kind(&self) -> u8 {
        match self {
            Self::NoteOn { .. } => 0x90,
            Self::NoteOff { .. } => 0x80,
            Self::ChangeInstrument { .. } => 0xc0,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::NoteOn { note, dynamics } => vec![self.kind(), note, dynamics],
            Self::NoteOff { note, dynamics } => vec![self.kind(), note, dynamics],
            Self::ChangeInstrument { code } => vec![self.kind(), code],
        }
    }
}

pub struct MidiEventDelta {
    delta_time: u8,
    event: MidiEvent,
}

impl MidiEventDelta {
    pub fn new(delta_time: u8, event: MidiEvent) -> Self {
        Self { delta_time, event }
    }

    pub fn size(&self) -> u16 {
        self.event.size() + 1
    }
}

impl IntoMidiData for MidiEventDelta {
    fn into_midi_data(self) -> crate::MidiData {
        let mut data = MidiData::new();
        data.push_byte(self.delta_time);
        data.push_bytes(self.event.into_bytes());
        data
    }
}
