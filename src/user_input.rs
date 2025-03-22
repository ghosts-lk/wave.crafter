use crate::synthesizer::{Synthesizer, Waveform};
use std::sync::{Arc, Mutex};

/*
pub fn handle_user_input(synth: Arc<Mutex<Synthesizer>>) {
    loop {
        println!("Enter command (freq [Hz], amp [0-1], wave [sine|square|triangle|sawtooth]):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut synth = synth.lock().unwrap();

        if input.starts_with("freq ") {
            if let Ok(freq) = input[5..].parse::<f32>() {
                synth.set_frequency(freq);
                println!("Frequency set to {} Hz", freq);
            } else {
                println!("Invalid frequency value.");
            }
        } else if input.starts_with("amp ") {
            if let Ok(amp) = input[4..].parse::<f32>() {
                if amp >= 0.0 && amp <= 1.0 {
                    synth.set_amplitude(amp);
                    println!("Amplitude set to {}", amp);
                } else {
                    println!("Amplitude must be between 0 and 1.");
                }
            } else {
                println!("Invalid amplitude value.");
            }
        } else if input.starts_with("wave ") {
            match &input[5..] {
                "sine" => {
                    synth.set_waveform(Waveform::Sine);
                    println!("Waveform set to Sine");
                }
                "square" => {
                    synth.set_waveform(Waveform::Square);
                    println!("Waveform set to Square");
                }
                "triangle" => {
                    synth.set_waveform(Waveform::Triangle);
                    println!("Waveform set to Triangle");
                }
                "sawtooth" => {
                    synth.set_waveform(Waveform::Sawtooth);
                    println!("Waveform set to Sawtooth");
                }
                _ => println!("Invalid waveform. Use sine, square, triangle, or sawtooth."),
            }
        } else {
            println!("Unknown command.");
        }
    }
}
*/
