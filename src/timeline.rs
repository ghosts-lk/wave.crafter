use serde::{Serialize, Deserialize};

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
    pub waveform: Waveform,
}
