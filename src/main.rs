mod synthesizer;
mod ui;

use synthesizer::{Synthesizer, Waveform};
use std::sync::{Arc, Mutex};

/// Entry point of the application
fn main() -> Result<(), eframe::Error> {
    let synth = Arc::new(Mutex::new(Synthesizer::new(440.0, 0.5, Waveform::Sine)));
    ui::run_ui(synth)
}
