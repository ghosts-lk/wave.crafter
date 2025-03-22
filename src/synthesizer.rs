use std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

pub struct Synthesizer {
    pub frequency: f32,
    pub amplitude: f32,
    pub waveform: Waveform,
}

impl Synthesizer {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        Self {
            frequency,
            amplitude,
            waveform,
        }
    }

    pub fn generate_sample(&self, time: f32) -> f32 {
        match self.waveform {
            Waveform::Sine => (2.0 * PI * self.frequency * time).sin(),
            Waveform::Square => {
                if (2.0 * PI * self.frequency * time).sin() >= 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Triangle => {
                2.0 * (2.0 * self.frequency * time - (2.0 * self.frequency * time).floor() - 0.5).abs()
                    - 1.0
            }
            Waveform::Sawtooth => 2.0 * (self.frequency * time - (self.frequency * time).floor()) - 1.0,
        } * self.amplitude
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn save_preset(&self) -> String {
        format!("{},{},{}", self.frequency, self.amplitude, self.waveform as u8)
    }

    pub fn load_preset(&mut self, preset: &str) {
        let parts: Vec<&str> = preset.split(',').collect();
        if parts.len() == 3 {
            if let Ok(freq) = parts[0].parse::<f32>() {
                self.frequency = freq;
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
}
