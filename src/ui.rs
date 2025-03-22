use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::synthesizer::{Synthesizer, Waveform};
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
                if ui.add(egui::Slider::new(&mut freq, 20.0..=2000.0)).changed() {
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

            // Track Management
            ui.heading("Tracks");
            if ui.button("Add Track").clicked() {
                println!("Adding a new track...");
                synth.add_track("New Track");
            }

            for track in &synth.tracks {
                ui.horizontal(|ui| {
                    ui.label(&track.id);
                    let mut volume = track.volume;
                    if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0)).changed() {
                        println!("Volume for {} set to {}", track.id, volume);
                    }
                    if ui.button("Mute").clicked() {
                        println!("Muted {}", track.id);
                    }
                });
            }

            ui.separator();
            ui.heading("Timeline");
            // Effects
            ui.heading("Effects");
            ui.horizontal(|ui| {
                ui.label("Reverb:");
                let mut reverb = synth.effects.reverb;
                if ui.add(egui::Slider::new(&mut reverb, 0.0..=100.0)).changed() {
                    synth.set_effect("reverb", reverb);
                }
            });
            ui.horizontal(|ui| {
                ui.label("Delay:");
                let mut delay = synth.effects.delay;
                if ui.add(egui::Slider::new(&mut delay, 0.0..=100.0)).changed() {
                    synth.set_effect("delay", delay);
                }
            });

            for clip in &mut synth.timeline.clips {
                ui.horizontal(|ui| {
                    ui.label(&clip.id);
                    ui.add(egui::Slider::new(&mut clip.start_time, 0.0..=10.0).text("Start Time"));
                    ui.add(egui::Slider::new(&mut clip.duration, 0.1..=5.0).text("Duration"));
                    if ui.button("Remove").clicked() {
                        synth.timeline.remove_clip(&clip.id);
                    }
                });
            }

            ui.separator();

            // Effects
            ui.heading("Effects");
            ui.horizontal(|ui| {
                ui.label("Reverb:");
                let mut reverb = synth.effects.reverb;
                if ui.add(egui::Slider::new(&mut reverb, 0.0..=100.0)).changed() {
                    synth.set_effect("reverb", reverb);
                }
            });
            ui.horizontal(|ui| {
                ui.label("Delay:");
                let mut delay = synth.effects.delay;
                if ui.add(egui::Slider::new(&mut delay, 0.0..=100.0)).changed() {
                    synth.set_effect("delay", delay);
                }
            });

            ui.separator();
            ui.heading("Project");

            if ui.button("💾 Export Audio").clicked() {
                println!("Exporting audio...");
                if let Err(e) = synth.export_to_wav(5.0, "output.wav") {
                    eprintln!("Failed to export audio: {}", e);
                }
            }

            if ui.button("Save Project").clicked() {
                if let Err(e) = synth.save_project("project.json") {
                    eprintln!("Failed to save project: {}", e);
                }
            }

            if ui.button("Load Project").clicked() {
                if let Err(e) = synth.load_project("project.json") {
                    eprintln!("Failed to load project: {}", e);
                }
            }
        });
    }
}
