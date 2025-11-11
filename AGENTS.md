# Gruvberry

A real-time audio frequency spectrum visualizer built in Rust with a beautiful VIBGYOR gradient.

## Features Implemented

### Core Functionality
- **Real-time FFT analysis** - 1024-point FFT for frequency analysis
- **Audio playback** - Plays WAV files using `rodio`
- **Live visualization** - Updates at ~60 FPS synchronized with playback
- **Automatic termination** - Stops when audio finishes or user quits

### Visualization
- **Smooth VIBGYOR gradient** - True color RGB gradient from Red (bass) to Violet (treble)
- **Logarithmic frequency scale** - More detail in bass, natural audio perception
- **Dynamic band count** - 76-156 frequency bands based on terminal width
- **Exponential smoothing** - Smooth transitions, less jumpy (smoothing factor: 0.3)
- **Minimum baseline** - Always shows at least 1 character per band for full rainbow visibility
- **High frequency boost** - Multiplies higher frequencies by up to 3x for better visibility

### Terminal UI (ratatui + crossterm)
- **Adaptive width** - 80-160 columns (including borders)
- **Left-aligned** - Stays at max 160 columns, empty space on right if wider
- **Dynamic layout** - Resizes in real-time as terminal is resized
- **Smart legends** - Shows frequency range indicators with partial segment support
- **Band details** - Displays frequency for each legend marker
- **Progress display** - Shows elapsed/total time and band count

### User Controls
- **q** or **Ctrl+C** - Immediately stops playback and exits
- **Auto-resize** - Visualization adapts to terminal size changes

## Technical Details

### Dependencies
```toml
ratatui = "0.29.0"   # Terminal UI framework
crossterm = "0.28"    # Terminal control
hound = "3.5"         # WAV file parsing
rodio = "0.21.1"      # Audio playback
rustfft = "6.1"       # FFT implementation
```

### Audio Processing
- **Sample rate**: 44.1kHz (or source file rate)
- **FFT size**: 1024 samples
- **Frequency range**: 20Hz - 22.05kHz (Nyquist)
- **Update rate**: ~60 FPS (16ms frame time)

### File Structure
- Hardcoded to `src/sound.wav` (modify in `main.rs` lines 468, 489)
- Supports WAV format only (via hound)

## Usage

```bash
# Place your WAV file
cp your_audio.wav src/sound.wav

# Run
cargo run

# Quit
# Press 'q' or Ctrl+C
```

## Next Steps / Potential Enhancements

### 1. Command-line Arguments
- Accept file path as argument
- Options for FFT size, smoothing factor, color themes
- Use `clap` crate for arg parsing

### 2. Multiple Color Themes
- Gruvbox dark/light
- Nord, Dracula, Solarized
- Monochrome/single color modes
- Press keys to switch themes in real-time

### 3. Enhanced Smoothing
- Adjustable smoothing factor
- Peak hold/decay indicators (dots that slowly fall)
- Different smoothing algorithms (moving average, etc.)

### 4. Stereo Visualization
- Split L/R channels
- Side-by-side or overlay display
- Phase correlation meter

### 5. Different Visualization Modes
- Bar chart (current)
- Waveform view (time domain)
- Circular/radial spectrum
- Spectrogram (scrolling frequency over time)
- Press 'm' to cycle through modes

### 6. Audio Format Support
- MP3, FLAC, OGG support (via symphonia)
- Playlist support
- Directory browsing

### 7. Export/Recording
- Save frames as images
- Record to animated GIF
- Export frequency data to CSV

### 8. Performance Optimizations
- Reduce allocations in render loop
- SIMD FFT optimizations
- GPU acceleration for visualization

### 9. Configuration File
- Save preferences (colors, smoothing, window size)
- TOML/YAML config support

### 10. Interactive Features
- Pause/resume playback
- Seek forward/backward
- Volume control
- Loop mode

## Learning Outcomes

This project demonstrates:
- **Rust ownership & borrowing** - Audio buffer sharing with `Arc<Mutex<>>`
- **Trait implementation** - Custom `Source` trait for sample capture
- **FFT & DSP** - Real-time frequency analysis
- **Terminal UI** - Full-featured TUI with ratatui
- **Concurrency** - Multi-threaded audio + visualization
- **Error handling** - `Result<>` types throughout
- **External crates** - Integration of audio/UI/math libraries
