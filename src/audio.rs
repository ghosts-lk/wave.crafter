use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{FftPlanner, num_complex::Complex};
use plotters::prelude::*;
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
                let time_step = 1.0 / config.sample_rate as f32;
                for sample in data.iter_mut() {
                    *sample = synth.generate_timeline_sample(time);
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

pub fn generate_spectrogram(samples: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&s| Complex { re: s, im: 0.0 }).collect();
    fft.process(&mut buffer);

    let magnitudes: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();

    let root = BitMapBackend::new("spectrogram.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Spectrogram", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..magnitudes.len(), 0.0..magnitudes.iter().cloned().fold(0.0, f32::max))?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        magnitudes.iter().enumerate().map(|(x, &y)| (x, y)),
        &BLUE,
    ))?;

    root.present()?;
    println!("Spectrogram saved to spectrogram.png");
    Ok(())
}
