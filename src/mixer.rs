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
                let raw_sample = track.volume * time.sin(); // Adjust mixing logic
                mixed_sample += raw_sample; // Sum raw samples
            }
        }
        mixed_sample // Return the mixed sample
    }

    pub fn apply_mixing(&self, time: f32) -> f32 {
        self.mix_tracks(time) // Use `mix_tracks`
    }
}
