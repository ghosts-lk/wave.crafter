mod synthesizer;
mod user_input;
mod ui;

use eframe::egui;
use synthesizer::{Synthesizer, Waveform};
use std::sync::{Arc, Mutex};
use std::thread;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait}; // Add missing imports

fn main() {
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
}

fn start_audio(synthesizer: Arc<Mutex<Synthesizer>>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to find a default output device");
    let config = device.default_output_config().unwrap();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), synthesizer.clone()),
        cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), synthesizer.clone()),
        cpal::SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), synthesizer.clone()),
    }
    .expect("Failed to build stream");

    stream.play().expect("Failed to play stream");

    // Keep the audio thread alive
    loop {
        std::thread::park();
    }
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    synthesizer: Arc<Mutex<Synthesizer>>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let synth = synthesizer.lock().unwrap();
            for sample in data.iter_mut() {
                let value = synth.generate_sample(sample_clock / sample_rate);
                sample_clock = (sample_clock + 1.0) % sample_rate;
                *sample = cpal::Sample::from::<f32>(&value);
            }
        },
        move |err| eprintln!("Stream error: {}", err),
    )
}
