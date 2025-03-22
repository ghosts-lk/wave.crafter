# WaveCraft

## Overview
WaveCraft is a Rust-based digital audio workstation (DAW) and synthesizer that allows users to create, mix, and export audio tracks. It features real-time waveform generation, timeline editing, and audio effects.

## Features
- **Real-time waveform rendering**: Visualize waveforms as you adjust parameters.
- **Binaural audio support**: Generate stereo audio with independent left and right frequencies.
- **Track management**: Add, mute, and adjust volume for multiple tracks.
- **Timeline editing**: Add and manage audio clips with customizable start times, durations, and waveforms.
- **Audio effects**: Apply effects like delay and reverb to your audio.
- **Spectrogram generation**: Visualize the frequency spectrum of audio samples.
- **Export functionality**: Save projects and export audio to WAV format.

## Performance Specifications
- **Audio Sample Rate**: 44.1 kHz
- **Latency**: Optimized for low-latency audio playback using `cpal`.
- **Parallel Processing**: Utilizes `rayon` for parallel sample generation, ensuring efficient performance on multi-core systems.
- **Memory Usage**: Designed to handle large audio projects with minimal memory overhead.
- **Export Speed**: Exports 5-minute audio tracks in under 10 seconds on modern hardware.

## Requirements
- Rust (latest stable version)
- `cargo-bundle` for packaging (install with `cargo install cargo-bundle`)

## Instructions

### Clone the Repository
```bash
git clone https://github.com/your-repo/wave.crafter.git
cd wave.crafter
```

### Build the Project
```bash
cargo build --release
```

### Run the Application
```bash
cargo run --release
```

### Export Audio
Export audio to a WAV file:
```bash
cargo run --release -- export
```

### Generate Spectrogram
Generate a spectrogram image:
```bash
cargo run --release -- spectrogram
```

### Package the Application
Create a standalone executable:
```bash
cargo bundle
```

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.