#[derive(Clone, Copy, Debug)]
pub enum Pitch {
    C4 = 60,
    D4 = 62,
    E4 = 64,
    F4 = 65,
    G4 = 67,
    A4 = 69,
    B4 = 71,
    C5 = 72,
    D5 = 74,
    E5 = 76,
    F5 = 77,
    G5 = 79,
    A5 = 81,
    B5 = 83,
    C6 = 84,
}

impl Pitch {
    pub fn frequency(&self) -> f32 {
        match self {
            Pitch::C4 => 261.63,
            Pitch::D4 => 293.66,
            Pitch::E4 => 329.63,
            Pitch::F4 => 349.23,
            Pitch::G4 => 392.,
            Pitch::A4 => 440.,
            Pitch::B4 => 493.88,
            Pitch::C5 => 523.25,
            Pitch::D5 => 587.33,
            Pitch::E5 => 659.26,
            Pitch::F5 => 698.46,
            Pitch::G5 => 783.99,
            Pitch::A5 => 880.,
            Pitch::B5 => 987.77,
            Pitch::C6 => 1046.5,
        }
    }
}
