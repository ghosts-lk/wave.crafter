use crate::synthesizer::{Synthesizer, Waveform};
use std::sync::{Arc, Mutex};
use egui::plot::{Line, Plot, PlotPoints};

pub struct SynthesizerApp {
    synthesizer: Arc<Mutex<Synthesizer>>,
    frequency_left: f32,
    frequency_right: f32, // Added for binaural audio
    amplitude: f32,
    waveform: Waveform,
    preset: String,
    paused: bool,
}

impl SynthesizerApp {
    pub fn new(synthesizer: Arc<Mutex<Synthesizer>>) -> Self {
        let synth = synthesizer.lock().unwrap();
        Self {
            synthesizer: synthesizer.clone(),
            frequency_left: synth.frequency_left,
            frequency_right: synth.frequency_right, // Initialize right frequency
            amplitude: synth.amplitude,
            waveform: synth.waveform,
            preset: String::new(),
            paused: false,
        }
    }
}

impl eframe::App for SynthesizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Synthesizer Controls");

            // Frequency sliders for binaural audio
            ui.horizontal(|ui| {
                ui.label("Left Frequency (Hz):");
                if ui
                    .add(egui::Slider::new(&mut self.frequency_left, 20.0..=2000.0))
                    .changed()
                {
                    self.synthesizer
                        .lock()
                        .unwrap()
                        .set_binaural_frequencies(self.frequency_left, self.frequency_right);
                }
            });
            ui.horizontal(|ui| {
                ui.label("Right Frequency (Hz):");
                if ui
                    .add(egui::Slider::new(&mut self.frequency_right, 20.0..=2000.0))
                    .changed()
                {
                    self.synthesizer
                        .lock()
                        .unwrap()
                        .set_binaural_frequencies(self.frequency_left, self.frequency_right);
                }
            });

            // Amplitude slider
            ui.horizontal(|ui| {
                ui.label("Amplitude:");
                if ui
                    .add(egui::Slider::new(&mut self.amplitude, 0.0..=1.0))
                    .changed()
                {
                    self.synthesizer
                        .lock()
                        .unwrap()
                        .set_amplitude(self.amplitude);
                }
            });

            // Waveform selection
            ui.horizontal(|ui| {
                ui.label("Waveform:");
                if ui
                    .radio_value(&mut self.waveform, Waveform::Sine, "Sine")
                    .clicked()
                {
                    self.synthesizer.lock().unwrap().set_waveform(Waveform::Sine);
                }
                if ui
                    .radio_value(&mut self.waveform, Waveform::Square, "Square")
                    .clicked()
                {
                    self.synthesizer.lock().unwrap().set_waveform(Waveform::Square);
                }
                if ui
                    .radio_value(&mut self.waveform, Waveform::Triangle, "Triangle")
                    .clicked()
                {
                    self.synthesizer.lock().unwrap().set_waveform(Waveform::Triangle);
                }
                if ui
                    .radio_value(&mut self.waveform, Waveform::Sawtooth, "Sawtooth")
                    .clicked()
                {
                    self.synthesizer.lock().unwrap().set_waveform(Waveform::Sawtooth);
                }
            });

            // Play/Pause buttons
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    self.paused = false;
                }
                if ui.button("Pause").clicked() {
                    self.paused = true;
                }
            });

            // Preset management
            ui.horizontal(|ui| {
                if ui.button("Save Preset").clicked() {
                    self.preset = self.synthesizer.lock().unwrap().save_preset();
                }
                if ui.button("Load Preset").clicked() {
                    self.synthesizer.lock().unwrap().load_preset(&self.preset);
                }
            });

            // Export to WAV button
            if ui.button("Export to WAV").clicked() {
                let synthesizer = self.synthesizer.lock().unwrap();
                if let Err(err) = synthesizer.export_to_wav(5.0, "output.wav") {
                    eprintln!("Failed to export audio: {}", err);
                }
            }

            // Enhanced waveform rendering
            ui.horizontal(|ui| {
                ui.label("Waveform Preview:");
                let points: Vec<[f32; 2]> = (0..1000)
                    .map(|i| {
                        let t = i as f32 / 1000.0;
                        [t, self.synthesizer.lock().unwrap().generate_sample(t, true)]
                    })
                    .collect();
                let points: PlotPoints = PlotPoints::from(
                    points.iter().map(|&[x, y]| [x as f64, y as f64]).collect::<Vec<_>>()
                );
                Plot::new("line_plot")
                    .show(ui, |plot_ui| {
                        plot_ui.line(Line::new(points));
                    });
            });
        });
    }
}
