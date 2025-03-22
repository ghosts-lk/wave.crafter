use eframe::egui::{self, ProgressBar}; // Import ProgressBar
use std::sync::{Arc, Mutex};
use crate::synthesizer::{Synthesizer, Waveform};
use std::thread;
use crate::audio::play_audio; // Updated import path

pub fn run_ui(synth: Arc<Mutex<Synthesizer>>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    // Start audio playback in a separate thread
    let synth_clone = Arc::clone(&synth);
    thread::spawn(move || {
        if let Err(e) = play_audio(synth_clone) { // Updated to call `play_audio` directly
            eprintln!("Audio playback error: {}", e);
        }
    });

    eframe::run_native(
        "Wave Crafter",
        options,
        Box::new(|_cc| Box::new(WaveCrafterApp {
            synth,
            loading: false, // Add loading state
            progress: 0.0,  // Add progress state
        })),
    )
}

struct WaveCrafterApp {
    synth: Arc<Mutex<Synthesizer>>,
    loading: bool, // Track if loading screen is active
    progress: f32, // Track progress for operations
}

impl eframe::App for WaveCrafterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.loading {
            // Show loading splash screen
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Loading...");
                ui.add(ProgressBar::new(self.progress).text(format!("{:.0}%", self.progress * 100.0)));
            });

            // Simulate progress (for demonstration purposes)
            self.progress += 0.01;
            if self.progress >= 1.0 {
                self.loading = false; // Stop loading when progress reaches 100%
                self.progress = 0.0;
            }
        } else {
            // Main UI
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("🎵 WaveCraft - Digital Audio Workstation");
                ui.separator();

                let mut synth = self.synth.lock().unwrap();

                // Update Mixer with tracks
                synth.mixer.tracks = synth.tracks.clone();

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

                // Add advanced track management
                ui.heading("Tracks");
                if ui.button("Add Track").clicked() {
                    synth.add_track("New Track");
                }

                for track in &mut synth.tracks {
                    ui.horizontal(|ui| {
                        ui.label(&track.id);
                        let mut volume = track.volume;
                        if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0)).changed() {
                            track.volume = volume;
                        }
                        if ui.button("Mute").clicked() {
                            track.muted = !track.muted;
                        }
                    });
                }

                // Add timeline visualization
                ui.heading("Timeline");
                let mut clips_to_remove = Vec::new(); // Collect clips to remove
                for clip in &mut synth.timeline.clips {
                    ui.horizontal(|ui| {
                        ui.label(&clip.id);
                        ui.add(egui::Slider::new(&mut clip.start_time, 0.0..=60.0).text("Start Time"));
                        ui.add(egui::Slider::new(&mut clip.duration, 0.1..=10.0).text("Duration"));
                        if ui.button("Remove").clicked() {
                            clips_to_remove.push(clip.id.clone()); // Clone clip ID to avoid borrowing issues
                        }
                    });
                }
                for clip_id in clips_to_remove {
                    synth.timeline.remove_clip(&clip_id); // Remove clips outside the loop
                }

                // Update Effects UI
                ui.heading("Effects");
                ui.horizontal(|ui| {
                    ui.label("Delay:");
                    let mut delay = self.synth.lock().unwrap().effects.delay;
                    if ui.add(egui::Slider::new(&mut delay, 0.0..=100.0)).changed() {
                        self.synth.lock().unwrap().update_effect("delay", delay); // Use `set_effect`
                    }
                });

                // Generate timeline sample for visualization
                let time = 0.0; // Example time value
                let sample = self.synth.lock().unwrap().apply_effects(time); // Use `generate_timeline_sample`
                ui.label(format!("Sample at time {}: {}", time, sample));

                // Generate mixed sample for visualization
                let mixed_sample = self.synth.lock().unwrap().generate_mixed_sample(time); // Use `generate_mixed_sample`
                ui.label(format!("Mixed Sample at time {}: {}", time, mixed_sample));

                ui.separator();
                ui.heading("Project");

                // Add export functionality
                if ui.button("💾 Export Project").clicked() {
                    if let Err(e) = self.synth.lock().unwrap().save_project("project.json") {
                        eprintln!("Failed to save project: {}", e);
                    }
                }

                // Export functionality with progress bar
                if ui.button("💾 Export Audio").clicked() {
                    self.loading = true; // Activate loading screen
                    let synth_clone = Arc::clone(&self.synth);
                    thread::spawn(move || {
                        if let Err(e) = synth_clone.lock().unwrap().export_to_wav(5.0, "output.wav") {
                            eprintln!("Failed to export audio: {}", e);
                        }
                    });
                }

                // Save project with progress bar
                if ui.button("Save Project").clicked() {
                    self.loading = true; // Activate loading screen
                    let synth_clone = Arc::clone(&self.synth);
                    thread::spawn(move || {
                        if let Err(e) = synth_clone.lock().unwrap().save_project("project.json") {
                            eprintln!("Failed to save project: {}", e);
                        }
                    });
                }

                // Load project with progress bar
                if ui.button("Load Project").clicked() {
                    self.loading = true; // Activate loading screen
                    let synth_clone = Arc::clone(&self.synth);
                    thread::spawn(move || {
                        if let Err(e) = synth_clone.lock().unwrap().load_project("project.json") {
                            eprintln!("Failed to load project: {}", e);
                        }
                    });
                }

                // Generate spectrogram with progress bar
                if ui.button("Generate Spectrogram").clicked() {
                    self.loading = true; // Activate loading screen
                    let samples = vec![0.0; 44100]; // Example samples
                    thread::spawn(move || {
                        crate::audio::process_audio(&samples);
                    });
                }
            });
        }
    }
}
