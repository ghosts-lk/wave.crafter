use serde::{Serialize, Deserialize};
use crate::synthesizer::Waveform; // Import Waveform for clip waveform type

/// Represents the timeline of the audio project, containing multiple audio clips.
#[derive(Serialize, Deserialize)]
pub struct Timeline {
    pub clips: Vec<Clip>, // List of audio clips in the timeline
}

impl Timeline {
    /// Creates a new, empty timeline.
    pub fn new() -> Self {
        Timeline {
            clips: Vec::new(), // Initialize with no clips
        }
    }
}

/// Represents an individual audio clip in the timeline.
#[derive(Serialize, Deserialize)]
pub struct Clip {
    pub id: String,        // Unique identifier for the clip
    pub start_time: f32,   // Start time of the clip in seconds
    pub duration: f32,     // Duration of the clip in seconds
    pub frequency: f32,    // Frequency of the waveform in the clip
    pub amplitude: f32,    // Amplitude of the waveform in the clip
    pub waveform: Waveform, // Waveform type (e.g., Sine, Square)
}
