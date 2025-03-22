use crate::synthesizer::{Synthesizer, Waveform};
use std::sync::{Arc, Mutex};
use egui::plot::{Line, PlotPoints}; // Correct import for `Values` equivalent in `egui`

pub struct SynthesizerApp {
    synthesizer: Arc<Mutex<Synthesizer>>,
    frequency: f32,
    amplitude: f32,
    waveform: Waveform,
    preset: String,
}

impl SynthesizerApp {
    pub fn new(synthesizer: Arc<Mutex<Synthesizer>>) -> Self {
        let synth = synthesizer.lock().unwrap();
        Self {
            synthesizer: synthesizer.clone(), // Clone instead of moving
            frequency: synth.frequency,
            amplitude: synth.amplitude,
            waveform: synth.waveform,
            preset: String::new(),
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let points: PlotPoints = /* ...generate points... */;
        ui.add(Line::new(points)); // Use `PlotPoints` instead of `Values`
    }
}

impl eframe::App for SynthesizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Synthesizer Controls");

            // Frequency slider
            ui.horizontal(|ui| {
                ui.label("Frequency (Hz):");
                if ui
                    .add(egui::Slider::new(&mut self.frequency, 20.0..=2000.0))
                    .changed()
                {
                    self.synthesizer
                        .lock()
                        .unwrap()
                        .set_frequency(self.frequency);
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

            // Preset management
            ui.horizontal(|ui| {
                if ui.button("Save Preset").clicked() {
                    self.preset = self.synthesizer.lock().unwrap().save_preset();
                }
                if ui.button("Load Preset").clicked() {
                    self.synthesizer.lock().unwrap().load_preset(&self.preset);
                }
            });

            // Waveform rendering
            ui.horizontal(|ui| {
                ui.label("Waveform Preview:");
                let points: Vec<[f32; 2]> = (0..1000)
                    .map(|i| {
                        let t = i as f32 / 1000.0;
                        [t, self.synthesizer.lock().unwrap().generate_sample(t)]
                    })
                    .collect();
                ui.add(Line::new(PlotPoints::from(points))); // Use `PlotPoints` instead of `Values`
            });
        });
    }
}
