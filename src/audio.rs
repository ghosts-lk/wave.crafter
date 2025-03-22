use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{FftPlanner, num_complex::Complex};
use plotters::prelude::*;
use std::sync::{Arc, Mutex};
use crate::synthesizer::Synthesizer;
use crossbeam_channel::{bounded, Receiver};

pub fn play_audio(synth: Arc<Mutex<Synthesizer>>) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host(); // Get the default audio host
    let device = host.default_output_device().ok_or("No output device available")?; // Get the default output device
    let supported_config = device.default_output_config()?; // Get the default output configuration
    let sample_format = supported_config.sample_format(); // Determine the sample format
    let config: cpal::StreamConfig = supported_config.into(); // Convert to a stream configuration

    let (sender, receiver): (crossbeam_channel::Sender<f32>, Receiver<f32>) = bounded(1024); // Create a channel for audio samples

    // Audio generation thread
    let synth_clone = Arc::clone(&synth);
    std::thread::spawn(move || {
        let mut time = 0.0; // Initialize time for sample generation
        let time_step = 1.0 / config.sample_rate.0 as f32; // Calculate the time step based on the sample rate
        loop {
            let synth = synth_clone.lock().unwrap(); // Lock the synthesizer for thread-safe access
            for _ in 0..1024 {
                let sample = synth.generate_mixed_sample(time); // Generate a mixed audio sample
                if sender.send(sample).is_err() {
                    return; // Exit if the receiver is dropped
                }
                time += time_step; // Increment time
            }
        }
    });

    // Audio playback thread
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = receiver.recv().unwrap_or(0.0); // Fetch samples from the channel
                }
            },
            |err| eprintln!("Stream error: {}", err), // Handle stream errors
            None,
        )?,
        _ => return Err("Unsupported stream format".into()), // Handle unsupported formats
    };

    stream.play()?; // Start audio playback
    std::thread::park(); // Keep the thread alive for audio playback
    Ok(())
}

pub fn generate_spectrogram(samples: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let mut planner = FftPlanner::new(); // Create an FFT planner
    let fft = planner.plan_fft_forward(samples.len()); // Plan a forward FFT
    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&s| Complex { re: s, im: 0.0 }).collect(); // Convert samples to complex numbers
    fft.process(&mut buffer); // Perform the FFT

    let magnitudes: Vec<f32> = buffer.iter().map(|c| c.norm()).collect(); // Calculate magnitudes

    let root = BitMapBackend::new("spectrogram.png", (800, 600)).into_drawing_area(); // Create a drawing area
    root.fill(&WHITE)?; // Fill the background with white
    let mut chart = ChartBuilder::on(&root)
        .caption("Spectrogram", ("sans-serif", 30)) // Add a chart caption
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..magnitudes.len(), 0.0..magnitudes.iter().cloned().fold(0.0, f32::max))?; // Set up the chart axes

    chart.configure_mesh().draw()?; // Draw the chart grid
    chart.draw_series(LineSeries::new(
        magnitudes.iter().enumerate().map(|(x, &y)| (x, y)), // Plot the magnitudes
        &BLUE,
    ))?;

    root.present()?; // Save the spectrogram image
    println!("Spectrogram saved to spectrogram.png"); // Log the output
    Ok(())
}

pub fn process_audio(samples: &[f32]) {
    if let Err(e) = generate_spectrogram(samples) { // Use `generate_spectrogram`
        eprintln!("Failed to generate spectrogram: {}", e); // Log errors
    }
}
