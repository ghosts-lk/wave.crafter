use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::synthesizer::{Synthesizer, Waveform};
use rfd::FileDialog;
use std::thread;

pub fn run_ui(synth: Arc<Mutex<Synthesizer>>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    // Start audio playback in a separate thread
    let synth_clone = Arc::clone(&synth);
    thread::spawn(move || {
        if let Err(e) = crate::audio::play_audio(synth_clone) {
            eprintln!("Audio playback error: {}", e);
        }
    });

    eframe::run_native(
        "Wave Crafter",
        options,
        Box::new(|_cc| Box::new(WaveCrafterApp { synth })),
    )
}

struct WaveCrafterApp {
    synth: Arc<Mutex<Synthesizer>>,
}

impl eframe::App for WaveCrafterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Wave Crafter");
            ui.separator();

            let mut synth = self.synth.lock().unwrap();

            // Frequency Slider
            ui.horizontal(|ui| {
                ui.label("Frequency (Hz):");
                let mut freq = synth.frequency_left;
                if ui.add(egui::Slider::new(&mut freq, 20.0..=20000.0)).changed() {
                    synth.set_binaural_frequencies(freq, freq);
                }
            });

            // Amplitude Slider
            ui.horizontal(|ui| {
                ui.label("Amplitude:");
                let mut amp = synth.amplitude;
                if ui.add(egui::Slider::new(&mut amp, 0.0..=1.0)).changed() {
                    synth.set_amplitude(amp);
                }
            });

            // Waveform Selector
            ui.horizontal(|ui| {
                ui.label("Waveform:");
                if ui.selectable_label(synth.waveform == Waveform::Sine, "🎵 Sine").clicked() {
                    synth.set_waveform(Waveform::Sine);
                }
                if ui.selectable_label(synth.waveform == Waveform::Square, "⬛ Square").clicked() {
                    synth.set_waveform(Waveform::Square);
                }
                if ui.selectable_label(synth.waveform == Waveform::Triangle, "🔺 Triangle").clicked() {
                    synth.set_waveform(Waveform::Triangle);
                }
                if ui.selectable_label(synth.waveform == Waveform::Sawtooth, "📐 Sawtooth").clicked() {
                    synth.set_waveform(Waveform::Sawtooth);
                }
            });

            ui.separator();

            // Spectrogram Placeholder
            ui.label("Spectrogram (Coming Soon)");

            // Export Button
            if ui.button("💾 Export Audio").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("WAV", &["wav"])
                    .add_filter("MP3", &["mp3"])
                    .add_filter("FLAC", &["flac"])
                    .set_title("Export Audio")
                    .save_file()
                {
                    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                    match extension {
                        "wav" => {
                            if let Err(e) = synth.export_to_wav(5.0, path.to_str().unwrap()) {
                                eprintln!("Failed to export WAV: {}", e);
                            }
                        }
                        "mp3" => {
                            println!("MP3 export is not implemented yet.");
                            // Add MP3 export logic here
                        }
                        "flac" => {
                            println!("FLAC export is not implemented yet.");
                            // Add FLAC export logic here
                        }
                        _ => eprintln!("Unsupported file format."),
                    }
                }
            }

            // Lock Spectrogram Button
            if ui.button("🔒 Lock Spectrogram").clicked() {
                println!("Spectrogram interaction locked.");
            }
        });
    }
}
