use std::f32::consts::PI;
use hound; // Add this crate to Cargo.toml for WAV file handling
use serde::{Serialize, Deserialize};

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
    pub timeline: Timeline, // Added timeline
}

pub struct Track {
    pub id: String,
    pub volume: f32,
    pub muted: bool,
}

pub struct Effects {,
    pub delay: f32,   pub delay: f32,
}}

#[derive(Serialize, Deserialize)]
pub struct AudioClip {ring,
    // ...existing code...
}
form,
#[derive(Serialize, Deserialize)]2,
pub struct Timeline {
    // ...existing code...
}
truct Timeline {
impl Synthesizer {    pub clips: Vec<AudioClip>,
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        Self {
            frequency_left: frequency,
            frequency_right: frequency, // Default to same frequency for both channels -> Self {
            amplitude, }
            waveform,
            tracks: Vec::new(), // Initialize tracks
            effects: Effects { reverb: 0.0, delay: 0.0 }, // Initialize effects clip: AudioClip) {
        }
    }

    pub fn generate_sample(&self, time: f32, is_left: bool) -> f32 {
        let frequency = if is_left {clip| clip.id != id);
            self.frequency_left
        } else {}
            self.frequency_right
        };
        let phase = 2.0 * PI * frequency * time;ub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        (match self.waveform {        Self {
            Waveform::Sine => phase.sin(),
            Waveform::Square => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },uency, // Default to same frequency for both channels
            Waveform::Triangle => 2.0 * (2.0 * frequency * time - (2.0 * frequency * time).floor() - 0.5).abs() - 1.0,       amplitude,
            Waveform::Sawtooth => 2.0 * (frequency * time - (frequency * time).floor()) - 1.0,            waveform,
        }) * self.amplitude
    }rb: 0.0, delay: 0.0 }, // Initialize effects
, // Initialize timeline
    pub fn set_amplitude(&mut self, amplitude: f32) {   }
        self.amplitude = amplitude;    }
    }
, time: f32, is_left: bool) -> f32 {
    pub fn set_waveform(&mut self, waveform: Waveform) {eft {
            self.frequency_leftwaveform;
        } else {
            self.frequency_right
        };ub fn set_binaural_frequencies(&mut self, left: f32, right: f32) {
        let phase = 2.0 * PI * frequency * time;        self.frequency_left = left;
        (match self.waveform {
            Waveform::Sine => phase.sin(),
            Waveform::Square => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },
            Waveform::Triangle => 2.0 * (2.0 * frequency * time - (2.0 * frequency * time).floor() - 0.5).abs() - 1.0,
            Waveform::Sawtooth => 2.0 * (frequency * time - (frequency * time).floor()) - 1.0,
        }) * self.amplitude   id: id.to_string(),
    }       volume: 0.5,
            muted: false,
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
, effect: &str, value: f32) {
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }  "delay" => self.effects.delay = value,

    pub fn set_binaural_frequencies(&mut self, left: f32, right: f32) {
        self.frequency_left = left;
        self.frequency_right = right;
    } filename: &str) -> Result<(), hound::Error> {

    pub fn add_track(&mut self, id: &str) {
        self.tracks.push(Track {
            id: id.to_string(),
            volume: 0.5,   sample_format: hound::SampleFormat::Int, // Fix: Add missing field
            muted: false,
        });t writer = hound::WavWriter::create(filename, spec)?;
    }   let sample_rate = spec.sample_rate as f32;
       let max_amplitude = i16::MAX as f32;
    pub fn set_effect(&mut self, effect: &str, value: f32) {        for i in 0..(duration * sample_rate) as usize {













































}    }        Ok(())        writer.finalize()?;        }            writer.write_sample(right_sample)?;            writer.write_sample(left_sample)?;            let right_sample = (self.generate_sample(time, false) * max_amplitude) as i16;            let left_sample = (self.generate_sample(time, true) * max_amplitude) as i16;            let time = i as f32 / sample_rate;        for i in 0..(duration * sample_rate) as usize {        let max_amplitude = i16::MAX as f32;        let sample_rate = spec.sample_rate as f32;        let mut writer = hound::WavWriter::create(filename, spec)?;        };            sample_format: hound::SampleFormat::Int, // Fix: Add missing field            bits_per_sample: 16,            sample_rate: 44100,            channels: 2, // Stereo for binaural audio        let spec = hound::WavSpec {    pub fn export_to_wav(&self, duration: f32, filename: &str) -> Result<(), hound::Error> {    }        sample        }            }                } * clip.amplitude;                    Waveform::Sawtooth => 2.0 * (clip.frequency * (time - clip.start_time) - (clip.frequency * (time - clip.start_time)).floor()) - 1.0,                    Waveform::Triangle => 2.0 * (2.0 * clip.frequency * (time - clip.start_time) - (2.0 * clip.frequency * (time - clip.start_time)).floor() - 0.5).abs() - 1.0,                    Waveform::Square => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },                    Waveform::Sine => phase.sin(),                sample += match clip.waveform {                let phase = 2.0 * PI * clip.frequency * (time - clip.start_time);            if time >= clip.start_time && time < clip.start_time + clip.duration {        for clip in &self.timeline.clips {        let mut sample = 0.0;    pub fn generate_timeline_sample(&self, time: f32) -> f32 {    }        }            _ => println!("Unknown effect: {}", effect),            "delay" => self.effects.delay = value,            "reverb" => self.effects.reverb = value,        match effect {            let time = i as f32 / sample_rate;
            let left_sample = (self.generate_sample(time, true) * max_amplitude) as i16;
            let right_sample = (self.generate_sample(time, false) * max_amplitude) as i16;
            writer.write_sample(left_sample)?;
            writer.write_sample(right_sample)?;
        }
        writer.finalize()?;
        Ok(())






}    }        Ok(())        self.timeline = serde_json::from_str(&json)?;        let json = std::fs::read_to_string(filename)?;    }


    pub fn load_project(&mut self, filename: &str) -> Result<(), std::io::Error> {
    pub fn save_project(&self, filename: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.timeline)?;
        std::fs::write(filename, json)?;
        Ok(())
    }