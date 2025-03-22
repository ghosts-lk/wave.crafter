# Wave Crafter

Wave Crafter is a digital audio workstation (DAW) built with Rust and Egui. It allows users to create, edit, and export audio tracks with various effects and features.

## Features
- Frequency and amplitude control
- Waveform selection (Sine, Square, Triangle, Sawtooth)
- Track management (Add, adjust volume, mute)
- Timeline visualization and clip management
- Audio effects (e.g., delay)
- Project export and import
- Audio export to WAV format

## Hardware Requirements
To run Wave Crafter smoothly, ensure your system meets the following minimum requirements:
- **Processor**: Dual-core CPU (2 GHz or faster)
- **Memory**: 4 GB RAM
- **Storage**: 500 MB of free disk space
- **Audio**: Sound card with JACK support
- **Operating System**: Linux (Debian/Ubuntu recommended)
- **Graphics**: OpenGL 3.3 compatible GPU

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/wave.crafter.git
   cd wave.crafter
   ```

2. Install dependencies:
   - Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).
   - Install the JACK audio library:
     ```bash
     sudo apt-get install libjack-jackd2-dev  # For Debian/Ubuntu
     ```

3. Build and run the application:
   ```bash
   cargo run --release
   ```

## Usage
- Adjust frequency and amplitude sliders to modify the sound.
- Select a waveform to change the audio signal shape.
- Add and manage tracks, clips, and effects.
- Export projects and audio files for later use.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgments
- Built with [Egui](https://github.com/emilk/egui) for the UI.
- Inspired by digital audio workstations like Audacity and FL Studio.