use crate::synthesizer::Track;

pub struct Mixer {
    pub tracks: Vec<Track>, // Store tracks for mixing
}

impl Mixer {
    pub fn new() -> Self {
        Mixer {
            tracks: Vec::new(), // Initialize with no tracks
        }
    }

    pub fn mix_tracks(&self, time: f32) -> f32 {
        let mut mixed_sample = 0.0;
        for track in &self.tracks {
            if !track.muted {
                // Generate a sine wave sample for the track and scale by volume
                let raw_sample = track.volume * (2.0 * std::f32::consts::PI * time).sin();
                mixed_sample += raw_sample; // Sum raw samples
            }
        }
        mixed_sample // Return the mixed sample
    }

    pub fn apply_mixing(&self, time: f32) -> f32 {
        self.mix_tracks(time) // Use `mix_tracks`
    }
}

impl Track {
    /// Creates a new track with default values.
    pub fn new(id: &str) -> Self {
        Track {
            id: id.to_string(),
            volume: 0.5, // Default volume
            muted: false, // Default muted state
        }
    }
}
