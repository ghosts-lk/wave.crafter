# Synthesizer

A modern synthesizer with a graphical user interface (GUI) for real-time sound generation and control.

## Features
- **Real-time waveform rendering**: Visualize the waveform as you adjust parameters.
- **Adjustable controls**: Modify frequency, amplitude, and waveform type (Sine, Square, Triangle, Sawtooth).
- **Preset management**: Save and load your favorite configurations.
- **Keyboard input**: Play notes dynamically using your computer keyboard.
- **Cross-platform support**: Works on Windows, macOS, and Linux.

## Requirements
- Rust (latest stable version)
- `cargo` build tool

## Dependencies
The following crates are used in this project:
- [`cpal`](https://crates.io/crates/cpal): For audio output.
- [`eframe`](https://crates.io/crates/eframe) and [`egui`](https://crates.io/crates/egui): For the graphical user interface.

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd tempest.dev
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the synthesizer:
   ```bash
   cargo run --release
   ```

## Usage
### GUI Controls
- **Frequency Slider**: Adjust the frequency of the waveform (20 Hz to 2000 Hz).
- **Amplitude Slider**: Control the amplitude (volume) of the waveform (0.0 to 1.0).
- **Waveform Selection**: Choose between Sine, Square, Triangle, and Sawtooth waveforms.
- **Preset Management**:
  - Click "Save Preset" to save the current configuration.
  - Click "Load Preset" to restore the saved configuration.
- **Waveform Preview**: View the real-time waveform rendering in the GUI.

### Keyboard Input
- Use the following keys to play notes dynamically:
  - `Z`, `X`, `C`, `V`, `B`, `N`, `M`: Play notes in the current octave.
  - `A`, `S`, `D`, `F`, `G`, `H`, `J`: Play sharp/flat notes.
  - `,` and `.`: Shift the octave down or up.

## Screenshots
![Synthesizer GUI](https://via.placeholder.com/800x400.png?text=Synthesizer+GUI+Preview)

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments
Special thanks to the Rust community and the authors of the `cpal` and `egui` crates for making this project possible.