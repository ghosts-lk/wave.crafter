# Wave Crafter

## Overview
Wave Crafter is a Rust-based audio synthesizer that generates waveforms.

## Features
- **Real-time waveform rendering**: Visualize the waveform as you adjust parameters.
- **Adjustable controls**: Modify frequency, amplitude, and waveform type (Sine, Square, Triangle, Sawtooth).
- **Preset management**: Save and load your favorite configurations.
- **Keyboard input**: Play notes dynamically using your computer keyboard.
- **Cross-platform support**: Works on Windows, macOS, and Linux.

## New Features
- **Dark Mode**: Toggle between light and dark themes.
- **Real-time Spectrogram**: Visualize audio frequencies in real-time.
- **Improved UI**: Added a dark mode toggle and spectrogram canvas.
- **Audio Export**: Save generated audio as `.wav` files.
- **Cross-Platform Desktop App**: Now available as a desktop application using Electron.js.

## Requirements
- Rust (latest stable version)
- `cpal` and `egui` crates

## Dependencies
The following crates are used in this project:
- [`cpal`](https://crates.io/crates/cpal): For audio output.
- [`eframe`](https://crates.io/crates/eframe) and [`egui`](https://crates.io/crates/egui): For the graphical user interface.

## Instructions
1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/wave.crafter.git
   cd wave.crafter
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run
   ```

4. Ensure your audio device supports the sample format used by the application.

## Build Instructions
1. Install dependencies:
   ```bash
   npm install
   ```

2. Run the app in development mode:
   ```bash
   npm run electron:serve
   ```

3. Build the desktop app:
   ```bash
   npm run electron:build
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
- **Dark Mode Toggle**: Switch between light and dark themes.
- **Real-time Spectrogram**: View the spectrogram in the canvas below the controls.

### Keyboard Input
- Use the following keys to play notes dynamically:
  - `Z`, `X`, `C`, `V`, `B`, `N`, `M`: Play notes in the current octave.
  - `A`, `S`, `D`, `F`, `G`, `H`, `J`: Play sharp/flat notes.
  - `,` and `.`: Shift the octave down or up.

## Troubleshooting
- If you encounter errors, ensure all dependencies are up-to-date:
  ```bash
  cargo update
  ```
- For unsupported sample formats, modify the `SampleFormat` match arms in `src/main.rs`.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments
Special thanks to the Rust community and the authors of the `cpal` and `egui` crates for making this project possible.