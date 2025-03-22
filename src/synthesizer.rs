use std::f32::consts::PI;
use hound;
use serde::{Serialize, Deserialize};
use crate::mixer::Mixer; // Import Mixer for track mixing
use rayon::prelude::*;   // Import Rayon for parallel processing
use crate::effects::Effects as AudioEffects; // Rename Effects to avoid conflicts

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Waveform {
    Sine,      // Sine wave
    Square,    // Square wave
    Triangle,  // Triangle wave
    Sawtooth,  // Sawtooth wave
}

pub struct Synthesizer {
    pub frequency_left: f32,  // Frequency for the left channel
    pub frequency_right: f32, // Frequency for the right channel
    pub amplitude: f32,       // Amplitude of the waveform
    pub waveform: Waveform,   // Current waveform type
    pub tracks: Vec<Track>,   // List of audio tracks
    pub effects: AudioEffects, // Audio effects (e.g., delay, reverb)
    pub timeline: Timeline,   // Timeline for managing audio clips
    pub mixer: Mixer,         // Mixer for combining tracks
}

impl Synthesizer {
    #[allow(dead_code)] // Suppress warning for unused function
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        Self {
            frequency_left: frequency,
            frequency_right: frequency,
            amplitude,
            waveform,
            tracks: Vec::new(), // Initialize with no tracks
            effects: AudioEffects { delay: 0.0, reverb: 0.0 }, // Default effects
            timeline: Timeline { clips: Vec::new() }, // Empty timeline
            mixer: Mixer::new(), // Initialize mixer
        }
    }

    /// Generates a single audio sample for the given time and channel (left or right).
    /// 
    /// # Parameters
    /// - `time`: The time at which the sample is generated.
    /// - `is_left`: Whether the sample is for the left channel.
    /// 
    /// # Returns
    /// - The generated audio sample.
    pub fn generate_sample(&self, time: f32, is_left: bool) -> f32 {
        let frequency = if is_left {
            self.frequency_left // Use left channel frequency
        } else {
            self.frequency_right // Use right channel frequency
        };
        let phase = 2.0 * PI * frequency * time; // Calculate phase
        return (match self.waveform {
            Waveform::Sine => phase.sin(), // Generate sine wave
            Waveform::Square => {
                if phase.sin() >= 0.0 { 1.0 } else { -1.0 } // Generate square wave
            }
            Waveform::Triangle => 2.0 * (2.0 * frequency * time - (2.0 * frequency * time).floor() - 0.5).abs() - 1.0, // Generate triangle wave
            Waveform::Sawtooth => 2.0 * (frequency * time - (frequency * time).floor()) - 1.0, // Generate sawtooth wave
        }) * self.amplitude; // Scale by amplitude
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn set_binaural_frequencies(&mut self, left: f32, right: f32) {
        self.frequency_left = left;
        self.frequency_right = right;
    }

    pub fn add_track(&mut self, id: &str) {
        self.tracks.push(Track {
            id: id.to_string(),
            volume: 0.5,
            muted: false,
        });
    }

    pub fn set_effect(&mut self, effect: &str, value: f32) {
        match effect {
            "delay" => self.effects.delay = value,
            "reverb" => self.effects.reverb = value, // Add handling for reverb
            _ => println!("Unknown effect: {}", effect),
        }
    }

    pub fn export_to_wav(&self, duration: f32, filename: &str) -> Result<(), hound::Error> {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(filename, spec)?;
        let sample_rate = spec.sample_rate as f32;
        let max_amplitude = i16::MAX as f32;

        // Generate samples in parallel
        let samples: Vec<(i16, i16)> = (0..(duration * sample_rate) as usize)
            .into_par_iter() // Parallel iterator
            .map(|i| {
                let time = i as f32 / sample_rate;
                let left_sample = (self.generate_sample(time, true) * max_amplitude) as i16;
                let right_sample = (self.generate_sample(time, false) * max_amplitude) as i16;
                (left_sample, right_sample)
            })
            .collect();

        // Write samples to the WAV file
        for (left_sample, right_sample) in samples {
            writer.write_sample(left_sample)?;
            writer.write_sample(right_sample)?;
        }

        writer.finalize()?;
        Ok(())
    }

    pub fn generate_timeline_sample(&self, time: f32) -> f32 {
        let mut sample = 0.0;
        for clip in &self.timeline.clips {
            if time >= clip.start_time && time < clip.start_time + clip.duration {
                let phase = 2.0 * PI * clip.frequency * (time - clip.start_time);
                sample += match clip.waveform {
                    Waveform::Sine => phase.sin(),
                    Waveform::Square => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },
                    Waveform::Triangle => 2.0 * (2.0 * clip.frequency * (time - clip.start_time) - (2.0 * clip.frequency * (time - clip.start_time)).floor() - 0.5).abs() - 1.0,
                    Waveform::Sawtooth => 2.0 * (clip.frequency * (time - clip.start_time) - (clip.frequency * (time - clip.start_time)).floor()) - 1.0,
                } * clip.amplitude;
            }
        }
        sample
    }

    pub fn save_project(&self, filename: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.timeline)?;
        std::fs::write(filename, json)?;
        Ok(())
    }

    pub fn load_project(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let json = std::fs::read_to_string(filename)?;
        self.timeline = serde_json::from_str(&json)?;
        Ok(())
    }

    pub fn apply_effects(&mut self, time: f32) -> f32 {
        let mut sample = self.generate_timeline_sample(time);
        sample = self.effects.apply(sample); // Use the apply method from AudioEffects
        sample
    }

    pub fn update_effect(&mut self, effect: &str, value: f32) {
        self.set_effect(effect, value);
    }

    pub fn generate_mixed_sample(&self, time: f32) -> f32 {
        let base_sample = self.generate_timeline_sample(time);
        let mixed_sample = self.mixer.apply_mixing(time);
        base_sample + mixed_sample
    }
}

#[derive(Clone)]
pub struct Track {
    pub id: String,
    pub volume: f32,
    pub muted: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Clip {
    pub id: String,
    pub start_time: f32,
    pub duration: f32,
    pub frequency: f32,
    pub amplitude: f32,
    pub waveform: Waveform,
}

#[derive(Serialize, Deserialize)]
pub struct Timeline {
    pub clips: Vec<Clip>,
}

impl Timeline {
    pub fn remove_clip(&mut self, clip_id: &str) {
        self.clips.retain(|clip| clip.id != clip_id);
    }
}