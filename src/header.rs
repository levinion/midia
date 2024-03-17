use crate::{IntoMidiData, MidiData};

pub enum TrackType {
    Single = 0,
    MultiSync = 1,
    MultiAsync = 2,
}

pub struct MidiHeader {
    typ: TrackType,
    track_number: u16,
    basic_time: u16,
}

impl MidiHeader {
    pub fn new(typ: TrackType, track_number: u16, basic_time: u16) -> Self {
        Self {
            typ,
            track_number,
            basic_time,
        }
    }
}

impl IntoMidiData for MidiHeader {
    fn into_midi_data(self) -> crate::MidiData {
        let mut data = MidiData::new();
        data.push_str("MThd");
        data.push_dword(6);
        data.push_word(self.typ as u16);
        data.push_word(self.track_number);
        data.push_word(self.basic_time);
        data
    }
}
