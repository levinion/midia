use crate::{
    event::{MidiEvent, MidiEventDelta},
    pitch::Pitch,
    IntoMidiData, MidiData,
};

pub struct MidiTrack {
    events: Vec<MidiEventDelta>,
}

#[allow(clippy::new_without_default)]
impl MidiTrack {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    fn data_size(&self) -> u32 {
        self.events
            .iter()
            .map(|event| event.size() as u32)
            .sum::<u32>()
            + 4
    }

    pub fn note_on(mut self, delta_time: u8, pitch: Pitch, dynamics: u8) -> Self {
        self.events.push(MidiEventDelta::new(
            delta_time,
            MidiEvent::NoteOn {
                note: pitch as u8,
                dynamics,
            },
        ));
        self
    }

    pub fn note_off(mut self, delta_time: u8, pitch: Pitch) -> Self {
        self.events.push(MidiEventDelta::new(
            delta_time,
            MidiEvent::NoteOff {
                note: pitch as u8,
                dynamics: 0,
            },
        ));
        self
    }

    pub fn note(self, delta_time: u8, pitch: Pitch, dynamics: u8) -> Self {
        self.note_on(0, pitch, dynamics).note_off(delta_time, pitch)
    }

    pub fn change_instrument(mut self, delta_time: u8, code: u8) -> Self {
        self.events.push(MidiEventDelta::new(
            delta_time,
            MidiEvent::ChangeInstrument { code },
        ));
        self
    }
}

impl IntoMidiData for MidiTrack {
    fn into_midi_data(self) -> crate::MidiData {
        let mut data = MidiData::new();
        data.push_str("MTrk");
        data.push_dword(self.data_size());
        self.events
            .into_iter()
            .for_each(|event| data.push_data(event));
        data.push_dword(0x00ff2f00);
        data
    }
}
