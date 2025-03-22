use eframe::egui::{self, ProgressBar};
use std::sync::{Arc, Mutex};
use crate::synthesizer::{Synthesizer, Waveform};
use std::thread;
use crate::audio::play_audio;

#[allow(dead_code)]
pub fn run_ui(synth: Arc<Mutex<Synthesizer>>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default(); // Default options for the eframe application

    // Start audio playback in a separate thread
    let synth_clone = Arc::clone(&synth);
    thread::spawn(move || {
        if let Err(e) = play_audio(synth_clone) {
            eprintln!("Audio playback error: {}", e); // Log any errors during audio playback
        }
    });

    eframe::run_native(
        "Wave Crafter", // Application title
        options,
        Box::new(|_cc| Box::new(WaveCrafterApp {
            synth,
            loading: false, // Initial state for loading
            progress: 0.0,  // Initial progress value
        })),
    )
}

#[allow(dead_code)]
struct WaveCrafterApp {
    synth: Arc<Mutex<Synthesizer>>, // Shared synthesizer instance
    loading: bool,                 // Loading state
    progress: f32,                 // Progress value for loading
}

impl eframe::App for WaveCrafterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.loading {
            self.show_loading_screen(ctx);
        } else {
            self.show_main_ui(ctx);
        }
    }
}

impl WaveCrafterApp {
    fn show_loading_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Loading..."); // Display loading message
            ui.add(ProgressBar::new(self.progress).text(format!("{:.0}%", self.progress * 100.0))); // Show progress bar
        });

        self.progress += 0.01; // Increment progress
        if self.progress >= 1.0 {
            self.loading = false; // Reset loading state
            self.progress = 0.0; // Reset progress
        }
    }

    fn show_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 WaveCraft - Digital Audio Workstation"); // Main application heading
            ui.separator(); // Add a separator line

            // Lock the synthesizer for thread-safe access
            let mut synth = self.synth.lock().unwrap();

            // Call methods that use the synthesizer
            self.show_frequency_slider(ui, &mut synth);
            self.show_amplitude_slider(ui, &mut synth);
            self.show_waveform_selector(ui, &mut synth);
            self.show_track_management(ui, &mut synth);
            self.show_timeline_visualization(ui, &mut synth);
            self.show_effects_ui(ui, &mut synth);

            // Drop the lock on `synth` before calling `self.show_project_management`
            drop(synth);

            // Call `show_project_management` separately
            self.show_project_management(ui);
        });
    }

    fn show_frequency_slider(&self, ui: &mut egui::Ui, synth: &mut Synthesizer) {
        ui.horizontal(|ui| {
            ui.label("Frequency (Hz):"); // Label for frequency slider
            let mut freq = synth.frequency_left;
            if ui.add(egui::Slider::new(&mut freq, 20.0..=2000.0)).changed() {
                synth.set_binaural_frequencies(freq, freq); // Update frequency
            }
        });
    }

    fn show_amplitude_slider(&self, ui: &mut egui::Ui, synth: &mut Synthesizer) {
        ui.horizontal(|ui| {
            ui.label("Amplitude:"); // Label for amplitude slider
            let mut amp = synth.amplitude;
            if ui.add(egui::Slider::new(&mut amp, 0.0..=1.0)).changed() {
                synth.set_amplitude(amp); // Update amplitude
            }
        });
    }

    fn show_waveform_selector(&self, ui: &mut egui::Ui, synth: &mut Synthesizer) {
        ui.horizontal(|ui| {
            ui.label("Waveform:"); // Label for waveform selector
            if ui.selectable_label(synth.waveform == Waveform::Sine, "🎵 Sine").clicked() {
                synth.set_waveform(Waveform::Sine); // Set waveform to sine
            }
            if ui.selectable_label(synth.waveform == Waveform::Square, "⬛ Square").clicked() {
                synth.set_waveform(Waveform::Square); // Set waveform to square
            }
            if ui.selectable_label(synth.waveform == Waveform::Triangle, "🔺 Triangle").clicked() {
                synth.set_waveform(Waveform::Triangle); // Set waveform to triangle
            }
            if ui.selectable_label(synth.waveform == Waveform::Sawtooth, "📐 Sawtooth").clicked() {
                synth.set_waveform(Waveform::Sawtooth); // Set waveform to sawtooth
            }
        });
    }

    fn show_track_management(&self, ui: &mut egui::Ui, synth: &mut Synthesizer) {
        ui.heading("Tracks"); // Heading for track management
        if ui.button("Add Track").clicked() {
            synth.add_track("New Track"); // Add a new track
        }

        for track in &mut synth.tracks {
            ui.horizontal(|ui| {
                ui.label(&track.id); // Display track ID
                let mut volume = track.volume;
                if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0)).changed() {
                    track.volume = volume; // Update track volume
                }
                if ui.button("Mute").clicked() {
                    track.muted = !track.muted; // Toggle mute state
                }
            });
        }
    }

    fn show_timeline_visualization(&self, ui: &mut egui::Ui, synth: &mut Synthesizer) {
        ui.heading("Timeline"); // Heading for timeline
        let mut clips_to_remove = Vec::new(); // Collect clips to remove
        for clip in &mut synth.timeline.clips {
            ui.horizontal(|ui| {
                ui.label(&clip.id); // Display clip ID
                ui.add(egui::Slider::new(&mut clip.start_time, 0.0..=60.0).text("Start Time")); // Adjust start time
                ui.add(egui::Slider::new(&mut clip.duration, 0.1..=10.0).text("Duration")); // Adjust duration
                if ui.button("Remove").clicked() {
                    clips_to_remove.push(clip.id.clone()); // Mark clip for removal
                }
            });
        }
        for clip_id in clips_to_remove {
            synth.timeline.remove_clip(&clip_id); // Remove marked clips
        }
    }

    fn show_effects_ui(&self, ui: &mut egui::Ui, synth: &mut Synthesizer) {
        ui.heading("Effects"); // Heading for effects
        ui.horizontal(|ui| {
            ui.label("Delay:"); // Label for delay effect
            let mut delay = synth.effects.delay;
            if ui.add(egui::Slider::new(&mut delay, 0.0..=100.0)).changed() {
                synth.update_effect("delay", delay); // Update delay effect
            }
        });
    }

    fn show_project_management(&mut self, ui: &mut egui::Ui) {
        ui.heading("Project"); // Heading for project management

        // Export Project
        if ui.button("💾 Export Project").clicked() {
            if let Err(e) = self.synth.lock().unwrap().save_project("project.json") {
                eprintln!("Failed to save project: {}", e); // Log errors during project export
            }
        }

        // Export Audio
        if ui.button("💾 Export Audio").clicked() {
            self.loading = true; // Set loading state
            let synth_clone = Arc::clone(&self.synth);
            thread::spawn(move || {
                if let Err(e) = synth_clone.lock().unwrap().export_to_wav(5.0, "output.wav") {
                    eprintln!("Failed to export audio: {}", e); // Log errors during audio export
                }
            });
        }

        // Load Project
        if ui.button("Load Project").clicked() {
            self.loading = true; // Set loading state
            let synth_clone = Arc::clone(&self.synth);
            thread::spawn(move || {
                if let Err(e) = synth_clone.lock().unwrap().load_project("project.json") {
                    eprintln!("Failed to load project: {}", e); // Log errors during project loading
                }
            });
        }
    }
}
