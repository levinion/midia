# MIDIA

A simple wrapper for writing the midi file easier.

## Usage

```rust
fn main() {
    let mut writer = MidiWriter::new(TrackType::Single, 1, 120);
    let mut track = MidiTrack::new();
    track
        .set_bpm(120)
        .set_tonality(Tonality::G)
        .change_instrument(InstrumentType::AcousticGrandPiano)
        .note(0.5, Pitch::A4, 90)
        .note(0.5, Pitch::B4, 90)
        .note(2., Pitch::C5, 90)
        .note(0.5, Pitch::B4, 90)
    // ...
        .note(1., Pitch::F5, 90)
        .note(1., Pitch::E5, 90)
        .note(1., Pitch::D5, 90)
        .note(1., Pitch::C5, 90)
        .repeat(5);
    writer.add_track(track);
    writer.write("...");
}
```
