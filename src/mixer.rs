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

    pub fn mix_tracks(&self, _time: f32) -> f32 {
        let mut mixed_sample = 0.0;
        for track in &self.tracks {
            if !track.muted {
                mixed_sample += track.volume; // Simplified mixing logic
            }
        }
        mixed_sample
    }
}
