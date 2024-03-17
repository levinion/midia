mod event;
mod header;
pub mod pitch;
pub mod track;

use std::{fmt::Display, fs::File, io::Write};

use crate::{
    header::{MidiHeader, TrackType},
    track::MidiTrack,
};

pub struct MidiWriter {
    data: MidiData,
}

impl MidiWriter {
    pub fn new(typ: TrackType, track_number: u16, basic_time: u16) -> Self {
        let mut data = MidiData::new();
        let header = MidiHeader::new(typ, track_number, basic_time);
        data.push_data(header);
        Self { data }
    }

    pub fn add_track(&mut self, track: MidiTrack) -> &mut Self {
        self.data.push_data(track);
        self
    }

    pub fn write(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.data.0).unwrap();
    }
}

impl Display for MidiData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_str = self
            .0
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<_>>();
        let str = data_str.join(" ");
        write!(f, "{}", str)
    }
}

#[derive(Default)]
pub struct MidiData(Vec<u8>);

impl MidiData {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push_str(&mut self, s: &str) {
        self.0.append(&mut s.as_bytes().to_vec());
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.0.push(byte);
    }

    pub fn push_word(&mut self, word: u16) {
        self.0.append(&mut word.to_be_bytes().to_vec());
    }

    pub fn push_dword(&mut self, dword: u32) {
        self.0.append(&mut dword.to_be_bytes().to_vec());
    }

    pub fn push_bytes(&mut self, mut bytes: Vec<u8>) {
        self.0.append(&mut bytes);
    }

    pub fn push_data(&mut self, data: impl IntoMidiData) {
        self.0.append(&mut data.into_midi_data().0);
    }
}

pub trait IntoMidiData {
    fn into_midi_data(self) -> MidiData;
}
