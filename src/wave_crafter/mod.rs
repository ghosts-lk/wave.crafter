pub mod mixer {
    use crate::synthesizer::Track;

    pub struct Mixer {
        pub tracks: Vec<Track>,
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
}

pub mod audio {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::{Arc, Mutex};
    use crate::synthesizer::Synthesizer;

    pub fn play_audio(synth: Arc<Mutex<Synthesizer>>) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or("No output device available")?;
        let supported_config = device.default_output_config()?;
        let sample_format = supported_config.sample_format();
        let config: cpal::StreamConfig = supported_config.into();

        let synth = Arc::clone(&synth);
        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    let synth = synth.lock().unwrap();
                    let mut time = 0.0;
                    let time_step = 1.0 / config.sample_rate.0 as f32;
                    for sample in data.iter_mut() {
                        *sample = synth.generate_sample(time, true); // Use Synthesizer's method
                        time += time_step;
                    }
                },
                |err| eprintln!("Stream error: {}", err),
                None,
            )?,
            _ => return Err("Unsupported stream format".into()),
        };

        stream.play()?;
        std::thread::park();
        Ok(())
    }
}
