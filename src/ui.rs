use eframe::egui;
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
        Box::new(|_cc| Box::new(WaveCrafterApp { synth })),
    )
}

struct WaveCrafterApp {
    synth: Arc<Mutex<Synthesizer>>,
}

impl eframe::App for WaveCrafterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                let mut delay = synth.effects.delay;wrap().effects.delay;
                if ui.add(egui::Slider::new(&mut delay, 0.0..=100.0)).changed() {
                    synth.effects.delay = delay;pdate_effect("delay", delay); // Use `set_effect`
                }
            });

            ui.separator();eline sample for visualization
            ui.heading("Project");mple time value
            let sample = self.synth.lock().unwrap().apply_effects(time); // Use `generate_timeline_sample`
            // Add export functionality time {}: {}", time, sample));
            if ui.button("💾 Export Project").clicked() {
                if let Err(e) = synth.save_project("project.json") {
                    eprintln!("Failed to save project: {}", e);
                }
            }/ Add export functionality
            if ui.button("💾 Export Project").clicked() {
            if ui.button("💾 Export Audio").clicked() {ject.json") {
                println!("Exporting audio...");roject: {}", e);
                if let Err(e) = synth.export_to_wav(5.0, "output.wav") {
                    eprintln!("Failed to export audio: {}", e);
                }
            }f ui.button("💾 Export Audio").clicked() {
                println!("Exporting audio...");
            if ui.button("Save Project").clicked() {5.0, "output.wav") {
                if let Err(e) = synth.save_project("project.json") {
                    eprintln!("Failed to save project: {}", e);
                }
            }
            if ui.button("Save Project").clicked() {
            if ui.button("Load Project").clicked() {project.json") {
                if let Err(e) = synth.load_project("project.json") {
                    eprintln!("Failed to load project: {}", e);
                }
            }
 if ui.button("Load Project").clicked() {
            if ui.button("Generate Spectrogram").clicked() {           if let Err(e) = synth.load_project("project.json") {
                let samples = vec![0.0; 44100]; // Example samples                   eprintln!("Failed to load project: {}", e);
                crate::audio::process_audio(&samples); // Use `generate_spectrogram`                }





}    }        });            }            }
        });
    }
}
