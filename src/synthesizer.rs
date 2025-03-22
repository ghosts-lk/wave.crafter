use std::f32::consts::PI;
use hound; // Add this crate to Cargo.toml for WAV file handling

#[derive(Clone, Copy, PartialEq, Debug)] // Added Debug and Copy
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

pub struct Synthesizer {
    pub frequency_left: f32,
    pub frequency_right: f32, // Added for binaural audio
    pub amplitude: f32,
    pub waveform: Waveform,
    pub tracks: Vec<Track>, // Added for track management
    pub effects: Effects,  // Added for effects
}

pub struct Track {
    pub id: String,
    pub volume: f32,
    pub muted: bool,
}

pub struct Effects {
    pub reverb: f32,
    pub delay: f32,
}

impl Synthesizer {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        Self {
            frequency_left: frequency,
            frequency_right: frequency, // Default to same frequency for both channels
            amplitude,
            waveform,
            tracks: Vec::new(), // Initialize tracks
            effects: Effects { reverb: 0.0, delay: 0.0 }, // Initialize effects
        }
    }

    pub fn generate_sample(&self, time: f32, is_left: bool) -> f32 {
        let frequency = if is_left {
            self.frequency_left
        } else {
            self.frequency_right
        };
        let phase = 2.0 * PI * frequency * time;
        (match self.waveform {
            Waveform::Sine => phase.sin(),
            Waveform::Square => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },
            Waveform::Triangle => 2.0 * (2.0 * frequency * time - (2.0 * frequency * time).floor() - 0.5).abs() - 1.0,
            Waveform::Sawtooth => 2.0 * (frequency * time - (frequency * time).floor()) - 1.0,
        }) * self.amplitude
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
            "reverb" => self.effects.reverb = value,
            "delay" => self.effects.delay = value,
            _ => println!("Unknown effect: {}", effect),
        }
    }

    pub fn export_to_wav(&self, duration: f32, filename: &str) -> Result<(), hound::Error> {
        let spec = hound::WavSpec {
            channels: 2, // Stereo for binaural audio
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int, // Fix: Add missing field
        };
        let mut writer = hound::WavWriter::create(filename, spec)?;
        let sample_rate = spec.sample_rate as f32;
        let max_amplitude = i16::MAX as f32;
        for i in 0..(duration * sample_rate) as usize {
            let time = i as f32 / sample_rate;
            let left_sample = (self.generate_sample(time, true) * max_amplitude) as i16;
            let right_sample = (self.generate_sample(time, false) * max_amplitude) as i16;
            writer.write_sample(left_sample)?;
            writer.write_sample(right_sample)?;
        }
        writer.finalize()?;
        Ok(())
    }
}
