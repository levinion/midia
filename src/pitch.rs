#[derive(Clone, Copy, Debug)]
pub enum Pitch {
    C4 = 60,
    D4 = 61,
    E4 = 62,
    F4 = 63,
    G4 = 64,
    A4 = 65,
    B4 = 66,
    C5 = 67,
    D5 = 68,
    E5 = 69,
    F5 = 70,
    G5 = 71,
    A5 = 72,
    B5 = 73,
    C6 = 74,
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
