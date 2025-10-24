use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use rustfft::{FftPlanner, num_complex::Complex};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{BarChart, Bar, Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// Custom wrapper that captures audio samples while playing
struct SampleCapture<I> {
    source: I,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
}

impl<I> SampleCapture<I> {
    fn new(source: I, sample_rate: u32) -> (Self, Arc<Mutex<Vec<f32>>>) {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let capture = SampleCapture {
            source,
            buffer: buffer.clone(),
            sample_rate,
        };
        (capture, buffer)
    }
}

impl<I> Iterator for SampleCapture<I>
where
    I: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.source.next() {
            // Store sample in buffer for FFT
            if let Ok(mut buf) = self.buffer.lock() {
                buf.push(sample);
                // Keep buffer size manageable (1024 samples for FFT)
                if buf.len() > 2048 {
                    buf.drain(0..1024);
                }
            }
            Some(sample)
        } else {
            None
        }
    }
}

impl<I> Source for SampleCapture<I>
where
    I: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}

// Perform FFT and visualize frequencies
fn visualize_frequencies(buffer: Arc<Mutex<Vec<f32>>>, sample_rate: u32, total_duration: f32) {
    use std::time::Instant;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1024);
    let start_time = Instant::now();

    // Number of frequency bands to display (0-20kHz split into bands)
    let num_bands = 20;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(50)); // Update ~20 times per second

        let elapsed = start_time.elapsed().as_secs_f32();
        if elapsed >= total_duration {
            break;
        }

        // Get samples from buffer
        let samples = {
            if let Ok(buf) = buffer.lock() {
                if buf.len() < 1024 {
                    continue;
                }
                buf.iter().rev().take(1024).rev().copied().collect::<Vec<f32>>()
            } else {
                continue;
            }
        };

        // Convert to complex numbers for FFT
        let mut complex_samples: Vec<Complex<f32>> = samples
            .iter()
            .map(|&s| Complex { re: s, im: 0.0 })
            .collect();

        // Perform FFT
        fft.process(&mut complex_samples);

        // Calculate magnitude for each frequency bin
        let magnitudes: Vec<f32> = complex_samples
            .iter()
            .take(512) // Only first half (Nyquist)
            .map(|c| (c.re * c.re + c.im * c.im).sqrt())
            .collect();

        // Group into frequency bands (0-20kHz)
        let max_freq = 20000.0;
        let freq_per_bin = sample_rate as f32 / 1024.0;
        let max_bin = (max_freq / freq_per_bin).min(512.0) as usize;

        let bin_size = max_bin / num_bands;
        let mut bands = vec![0.0f32; num_bands];

        for (i, band) in bands.iter_mut().enumerate() {
            let start = i * bin_size;
            let end = ((i + 1) * bin_size).min(magnitudes.len());
            if start < end {
                *band = magnitudes[start..end].iter().sum::<f32>() / (end - start) as f32;
            }
        }

        // Normalize bands
        let max_amplitude = bands.iter().cloned().fold(0.0f32, f32::max).max(1.0);

        // Print visualization
        print!("\r"); // Return to start of line

        for (i, &band) in bands.iter().enumerate() {
            let normalized = (band / max_amplitude * 8.0) as usize;
            let bar = "█".repeat(normalized.min(8));
            print!("{:<8}", bar);

            // Add separator every 5 bands for readability
            if (i + 1) % 5 == 0 && i < num_bands - 1 {
                print!("│");
            }
        }

        print!(" [{:.2}/{:.2}s]", elapsed, total_duration);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    println!(); // New line after visualization ends
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the WAV file
    let file = File::open("src/sound.wav")?;
    let reader = BufReader::new(file);

    // Parse WAV metadata
    let wav_reader = hound::WavReader::new(reader)?;
    let spec = wav_reader.spec();

    // Calculate duration
    let duration = wav_reader.duration() as f32 / spec.sample_rate as f32;

    println!("WAV File Loaded!");
    println!("Sample Rate: {} Hz", spec.sample_rate);
    println!("Channels: {}", spec.channels);
    println!("Duration: {:.2} seconds", duration);

    // Create audio output stream
    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(&stream_handle.mixer());

    // Open file again for playback (we consumed the first one)
    let sample_rate = spec.sample_rate;
    let file = File::open("src/sound.wav")?;
    let source = Decoder::new(BufReader::new(file))?;
    let source = rodio::source::UniformSourceIterator::new(source, 1, sample_rate);

    // Wrap source with our sample capture
    let (wrapped_source, sample_buffer) = SampleCapture::new(source, sample_rate);

    println!("\nPlaying wav file... sound.wav\n");

    // Add audio to sink and play
    sink.append(wrapped_source);

    // Spawn thread to perform FFT and display
    let handle = std::thread::spawn(move || {
        visualize_frequencies(sample_buffer, sample_rate, duration);
    });

    // Wait until playback finishes
    sink.sleep_until_end();

    // Wait for visualization thread
    handle.join().unwrap();

    println!("\nWav file playing completed");

    Ok(())
}
