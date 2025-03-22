# WaveCraft

## Overview
WaveCraft is a Rust-based audio synthesizer that generates waveforms.

## Features
- **Real-time waveform rendering**: Visualize the waveform as you adjust parameters.
- **Adjustable controls**: Modify frequency, amplitude, and waveform type (Sine, Square, Triangle, Sawtooth).
- **Cross-platform support**: Works on Windows, macOS, and Linux.
- **Standalone executable**: Package the app for distribution.

## Requirements
- Rust (latest stable version)
- `cargo-bundle` for packaging (install with `cargo install cargo-bundle`)

## Instructions
1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/wave.crafter.git
   cd wave.crafter
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

4. Package the application as a standalone executable:
   ```bash
   cargo bundle
   ```

5. Ensure your audio device supports the sample format used by the application.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.