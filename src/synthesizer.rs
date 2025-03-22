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
}

impl Synthesizer {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        Self {
            frequency_left: frequency,
            frequency_right: frequency, // Default to same frequency for both channels
            amplitude,
            waveform,
        }
    }

    pub fn generate_sample(&self, time: f32, is_left: bool) -> f32 {
        let frequency = if is_left {
            self.frequency_left
        } else {
            self.frequency_right
        };
        (match self.waveform {
            Waveform::Sine => (2.0 * PI * frequency * time).sin(),
            Waveform::Square => {
                if (2.0 * PI * frequency * time).sin() >= 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Triangle => {
                2.0 * (2.0 * frequency * time - (2.0 * frequency * time).floor() - 0.5).abs() - 1.0
            }
            Waveform::Sawtooth => {
                2.0 * (frequency * time - (frequency * time).floor()) - 1.0
            }
        }) * self.amplitude
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency_left = frequency;
        self.frequency_right = frequency;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn save_preset(&self) -> String {
        format!("{},{},{}", self.frequency_left, self.amplitude, self.waveform as u8)
    }

    pub fn load_preset(&mut self, preset: &str) {
        let parts: Vec<&str> = preset.split(',').collect();
        if parts.len() == 3 {
            if let Ok(freq) = parts[0].parse::<f32>() {
                self.frequency_left = freq;
                self.frequency_right = freq;
            }
            if let Ok(amp) = parts[1].parse::<f32>() {
                self.amplitude = amp;
            }
            if let Ok(wave) = parts[2].parse::<u8>() {
                self.waveform = match wave {
                    0 => Waveform::Sine,
                    1 => Waveform::Square,
                    2 => Waveform::Triangle,
                    3 => Waveform::Sawtooth,
                    _ => self.waveform,
                };
            }
        }
    }

    pub fn max_amplitude(&self) -> f32 {
        // Return the maximum amplitude value (adjust as needed)
        1.0
    }

    pub fn set_binaural_frequencies(&mut self, left: f32, right: f32) {
        self.frequency_left = left;
        self.frequency_right = right;
    }

    pub fn export_to_wav(&self, duration: f32, filename: &str) -> Result<(), hound::Error> {
        let spec = hound::WavSpec {
            channels: 2, // Stereo for binaural audio
            sample_rate: 44100,
            bits_per_sample: 16,
        };
        let mut writer = hound::WavWriter::create(filename, spec)?;
        let sample_rate = spec.sample_rate as f32;
        for i in 0..(duration * sample_rate) as usize {
            let time = i as f32 / sample_rate;
            let left_sample = (self.generate_sample(time, true) * i16::MAX as f32) as i16;
            let right_sample = (self.generate_sample(time, false) * i16::MAX as f32) as i16;
            writer.write_sample(left_sample)?;
            writer.write_sample(right_sample)?;
        }
        writer.finalize()?;
        Ok(())
    }
}
