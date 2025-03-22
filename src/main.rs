mod synthesizer;
mod user_input;
mod ui;

use synthesizer::{Synthesizer, Waveform};
use std::sync::{Arc, Mutex};
use std::thread;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Entry point of the application
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shared synthesizer instance
    let synthesizer = Arc::new(Mutex::new(Synthesizer::new(440.0, 0.5, Waveform::Sine)));

    // Spawn a thread for audio playback
    let synth_clone = synthesizer.clone();
    thread::spawn(move || {
        start_audio(synth_clone);
    });

    // Start the GUI
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Synthesizer UI",
        options,
        Box::new(|_cc| Box::new(ui::SynthesizerApp::new(synthesizer))),
    );

    Ok(())
}

/// Starts the audio playback thread
fn start_audio(synthesizer: Arc<Mutex<Synthesizer>>) {
    // Get the default audio host and output device
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to find a default output device");
    let config = device.default_output_config().unwrap();

    // Match the sample format and build the appropriate audio stream
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), synthesizer.clone()),
        cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), synthesizer.clone()),
        cpal::SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), synthesizer.clone()),
        _ => todo!(),
    }
    .expect("Failed to build stream");

    // Start the audio stream
    stream.play().expect("Failed to play stream");

    // Keep the audio thread alive indefinitely
    loop {
        std::thread::park();
    }
}

/// Builds an audio stream for the given sample type `T`
fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    synthesizer: Arc<Mutex<Synthesizer>>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + cpal::FromSample<f32> + cpal::SizedSample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    let mut time = 0.0;
    let time_step = 1.0 / sample_rate;

    device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let synthesizer = synthesizer.lock().unwrap();
            for frame in data.chunks_mut(channels) {
                let left_value: f32 = synthesizer.generate_sample(time, true);
                let right_value: f32 = synthesizer.generate_sample(time, false);
                time += time_step;
                let left_sample: T = T::from_sample(left_value);
                let right_sample: T = T::from_sample(right_value);
                frame[0] = left_sample;
                if channels > 1 {
                    frame[1] = right_sample;
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )
}
