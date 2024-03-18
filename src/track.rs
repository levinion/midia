use crate::{
    event::{MidiEvent, MidiEventDelta},
    pitch::Pitch,
    typ::{InstrumentType, Tonality},
    IntoMidiData, MidiData,
};

pub struct MidiTrack {
    events: Vec<MidiEventDelta>,
    tonality: Tonality,
    bpm: u16,
}

#[allow(clippy::new_without_default)]
impl MidiTrack {
    pub fn new() -> Self {
        Self {
            events: vec![],
            tonality: Tonality::C,
            bpm: 120,
        }
    }

    pub fn set_tonality(mut self, tonality: Tonality) -> Self {
        self.tonality = tonality;
        self
    }

    pub fn set_bpm(mut self, bpm: u16) -> Self {
        self.bpm = bpm;
        self
    }

    fn data_size(&self) -> u32 {
        self.events
            .iter()
            .map(|event| event.size() as u32)
            .sum::<u32>()
            + 4 // end flag size
    }

    pub fn note_on(mut self, beats: f32, pitch: Pitch, velocity: u8) -> Self {
        self.events.push(MidiEventDelta::new(
            (beats * self.bpm as f32) as u16,
            MidiEvent::NoteOn {
                note: (pitch as u8 + self.tonality as u8),
                velocity,
            },
        ));
        self
    }

    pub fn note_off(mut self, beats: f32, pitch: Pitch) -> Self {
        self.events.push(MidiEventDelta::new(
            (beats * self.bpm as f32) as u16,
            MidiEvent::NoteOff {
                note: pitch as u8,
                velocity: 0,
            },
        ));
        self
    }

    pub fn note(self, beats: f32, pitch: Pitch, velocity: u8) -> Self {
        self.note_on(0., pitch, velocity).note_off(beats, pitch)
    }

    pub fn change_instrument(mut self, instrument: InstrumentType) -> Self {
        self.events.push(MidiEventDelta::new(
            0,
            MidiEvent::ProgramChange {
                code: instrument as u8,
            },
        ));
        self
    }

    pub fn note_after_push(mut self, delta_time: u16, pitch: Pitch, amount: u8) -> Self {
        self.events.push(MidiEventDelta::new(
            delta_time,
            MidiEvent::NoteAfterTouch {
                note: pitch as u8,
                amount,
            },
        ));
        self
    }

    pub fn repeat(mut self, n: usize) -> Self {
        self.events = self.events.repeat(n);
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
