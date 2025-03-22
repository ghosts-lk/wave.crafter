use serde::{Serialize, Deserialize};
use crate::synthesizer::Waveform; // Import Waveform

#[derive(Serialize, Deserialize)]
pub struct Timeline {
    pub clips: Vec<Clip>,
}

#[derive(Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub start_time: f32,
    pub duration: f32,
    pub frequency: f32,
    pub amplitude: f32,
    pub waveform: Waveform, // Use imported Waveform
}
