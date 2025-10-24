use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use rustfft::{FftPlanner, num_complex::Complex};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style},
    text::{Line, Span},
    Terminal,
};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::atomic::{AtomicBool, Ordering};

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

// Map frequency index to smooth VIBGYOR gradient (true color)
fn frequency_to_color(index: usize, total: usize) -> Color {
    // Ensure we don't divide by zero
    let total = total.max(1);
    let ratio = index as f32 / (total - 1) as f32; // 0.0 to 1.0

    // Smooth gradient: Red -> Orange -> Yellow -> Green -> Cyan -> Blue -> Violet
    // Using HSV-like interpolation for smooth transitions

    if ratio < 0.167 {
        // Red (0%) -> Orange (16.7%)
        let t = ratio / 0.167;
        Color::Rgb(255, (165.0 * t) as u8, 0)
    } else if ratio < 0.333 {
        // Orange (16.7%) -> Yellow (33.3%)
        let t = (ratio - 0.167) / 0.166;
        Color::Rgb(255, (165.0 + 90.0 * t) as u8, 0)
    } else if ratio < 0.5 {
        // Yellow (33.3%) -> Green (50%)
        let t = (ratio - 0.333) / 0.167;
        Color::Rgb((255.0 * (1.0 - t)) as u8, 255, 0)
    } else if ratio < 0.667 {
        // Green (50%) -> Cyan (66.7%)
        let t = (ratio - 0.5) / 0.167;
        Color::Rgb(0, 255, (255.0 * t) as u8)
    } else if ratio < 0.833 {
        // Cyan (66.7%) -> Blue (83.3%)
        let t = (ratio - 0.667) / 0.166;
        Color::Rgb(0, (255.0 * (1.0 - t)) as u8, 255)
    } else {
        // Blue (83.3%) -> Violet (100%)
        let t = (ratio - 0.833) / 0.167;
        Color::Rgb((148.0 * t) as u8, 0, (255.0 - 44.0 * t) as u8)
    }
}

// Perform FFT and visualize frequencies with ratatui
fn visualize_frequencies(
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
    total_duration: f32,
    should_stop: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1024);
    let start_time = Instant::now();

    // Dynamic number of bands based on terminal width (will be updated each frame)
    let mut num_bands = 60;
    let mut smoothed_bands = vec![0.0f32; num_bands];

    loop {
        // Check for Ctrl+C or 'q' key
        if poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key) = read()? {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)
                    || key.code == KeyCode::Char('q')
                {
                    should_stop.store(true, Ordering::Relaxed);
                    break;
                }
            }
        }

        let elapsed = start_time.elapsed().as_secs_f32();
        if elapsed >= total_duration || should_stop.load(Ordering::Relaxed) {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS

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

        // Will be calculated after we know terminal width
        let mut bands = vec![0.0f32; num_bands];
        let freq_per_bin = sample_rate as f32 / 1024.0;

        // Define logarithmic frequency ranges (more bins for low freq, fewer for high)
        let min_freq: f32 = 20.0; // Human hearing starts ~20 Hz
        let max_freq: f32 = (sample_rate / 2) as f32; // Nyquist frequency (22.05kHz for 44.1kHz)
        let log_min = min_freq.ln();
        let log_max = max_freq.ln();

        for (i, band) in bands.iter_mut().enumerate() {
            // Calculate logarithmic frequency range for this band
            let log_start = log_min + (i as f32 / num_bands as f32) * (log_max - log_min);
            let log_end = log_min + ((i + 1) as f32 / num_bands as f32) * (log_max - log_min);

            let freq_start = log_start.exp();
            let freq_end = log_end.exp();

            let bin_start = (freq_start / freq_per_bin) as usize;
            let bin_end = (freq_end / freq_per_bin).min(512.0) as usize;

            if bin_start < bin_end && bin_end <= magnitudes.len() {
                // Average magnitude in this frequency range
                *band = magnitudes[bin_start..bin_end].iter().sum::<f32>() / (bin_end - bin_start) as f32;

                // Apply slight boost to higher frequencies for better visibility
                let boost = 1.0 + (i as f32 / num_bands as f32) * 2.0;
                *band *= boost;
            }
        }

        // Apply exponential smoothing (0.3 = smooth, 0.7 = responsive)
        let smoothing_factor = 0.3;
        for (i, &new_value) in bands.iter().enumerate() {
            smoothed_bands[i] = smoothed_bands[i] * (1.0 - smoothing_factor) + new_value * smoothing_factor;
        }

        // Normalize bands
        let max_amplitude = smoothed_bands.iter().cloned().fold(0.0f32, f32::max).max(1.0);

        // Normalize to 0-100 range for visualization
        let normalized_bands: Vec<f32> = smoothed_bands
            .iter()
            .map(|&band| (band / max_amplitude) * 100.0)
            .collect();

        // Calculate num_bands BEFORE terminal.draw to update smoothed_bands size
        let current_size = terminal.size().unwrap_or_else(|_| ratatui::layout::Size { width: 80, height: 24 });
        let current_width = current_size.width;

        // Calculate based on terminal width (80-160 total columns including borders)
        const MIN_WIDTH: u16 = 80;
        const MAX_WIDTH: u16 = 160;

        let (calculated_num_bands, num_legend_bands) = if current_width >= MIN_WIDTH {
            let usable_width = current_width.min(MAX_WIDTH).saturating_sub(4); // Cap at 160 total, then exclude borders
            let bands = usable_width as usize; // 1 column = 1 band

            // Legend bands: 1 legend per 10 characters (8-16 legends total)
            let legends = (usable_width / 10).max(8).min(16) as usize;

            (bands, legends)
        } else {
            // Terminal too small, keep current values and show warning
            (num_bands, 8)
        };

        // Resize smoothed_bands if terminal width changed
        if calculated_num_bands != num_bands {
            num_bands = calculated_num_bands;
            smoothed_bands.resize(num_bands, 0.0);
            bands.resize(num_bands, 0.0);
        }

        // Render UI
        terminal.draw(|f| {
            let terminal_width = f.area().width;
            let terminal_height = f.area().height;

            // Minimum terminal size requirement
            const MIN_WIDTH: u16 = 80; // Total including borders
            const MIN_HEIGHT: u16 = 20;
            const MAX_DISPLAY_WIDTH: u16 = 160; // Max total width including borders

            if terminal_width < MIN_WIDTH || terminal_height < MIN_HEIGHT {
                // Terminal too small - show warning
                let warning_text = format!(
                    "Terminal too small!\n\n\
                     Current size: {}x{}\n\
                     Minimum required: {}x{}\n\n\
                     Please resize your terminal window.",
                    terminal_width, terminal_height, MIN_WIDTH, MIN_HEIGHT
                );
                let warning_widget = Paragraph::new(warning_text)
                    .block(Block::default().borders(Borders::ALL).title("Error"))
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Red));
                f.render_widget(warning_widget, f.area());
                return;
            }

            // Constrain display area to max width, left-aligned
            let display_width = terminal_width.min(MAX_DISPLAY_WIDTH);
            let display_area = ratatui::layout::Rect {
                x: f.area().x,
                y: f.area().y,
                width: display_width,
                height: terminal_height,
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),     // Frequency spectrum (main visualization)
                    Constraint::Length(3),   // Legend indicators
                    Constraint::Length(num_legend_bands.min(10) as u16 / 2 + 3), // Legend details (dynamic height)
                    Constraint::Length(3),   // Time progress
                ].as_ref())
                .split(display_area);

            // Calculate actual usable width for spectrum (exclude borders)
            let spectrum_width = chunks[0].width.saturating_sub(2) as usize; // Subtract borders
            let spectrum_height = chunks[0].height.saturating_sub(2) as usize;

            // Build spectrum as text lines (row by row, from top to bottom)
            let mut spectrum_lines: Vec<Line> = Vec::new();

            for row in (0..spectrum_height).rev() {
                let mut spans: Vec<Span> = Vec::new();

                for col in 0..spectrum_width {
                    // Map screen column to frequency band index
                    let band_index = ((col as f64 / spectrum_width as f64) * num_bands as f64) as usize;
                    if band_index >= normalized_bands.len() {
                        spans.push(Span::raw(" "));
                        continue;
                    }

                    let amplitude = normalized_bands[band_index];
                    let color = frequency_to_color(band_index, num_bands);

                    // Calculate how high this bar should be (1-spectrum_height, minimum 1)
                    let bar_height = ((amplitude / 100.0) * spectrum_height as f32) as usize;
                    let bar_height = bar_height.max(1); // Always show at least 1 character

                    // If this row is below the bar height, draw a block
                    if row < bar_height {
                        spans.push(Span::styled("█", Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw(" "));
                    }
                }

                spectrum_lines.push(Line::from(spans));
            }

            let spectrum = Paragraph::new(spectrum_lines)
                .block(
                    Block::default()
                        .title(format!("Gruvberry - Frequency Spectrum (20Hz - {:.1}kHz) VIBGYOR", max_freq / 1000.0))
                        .borders(Borders::ALL),
                );

            f.render_widget(spectrum, chunks[0]);

            // Legend indicators (|---1---|---2---|...) - must match spectrum_width exactly
            let legend_width = chunks[1].width.saturating_sub(2) as usize; // Match legend box width
            let mut legend_spans: Vec<Span> = Vec::new();

            let segment_width = legend_width / num_legend_bands;
            let remainder_width = legend_width % num_legend_bands; // Partial segment at the end

            for i in 0..num_legend_bands {
                let num_str = (i + 1).to_string();
                let num_len = num_str.len();

                // Build segment: |---N---
                legend_spans.push(Span::raw("|"));

                // Account for variable digit length (1-9 = 1 char, 10-20 = 2 chars)
                let padding = segment_width.saturating_sub(num_len + 1); // 1 for |
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;

                if left_pad > 0 {
                    legend_spans.push(Span::raw("─".repeat(left_pad)));
                }
                legend_spans.push(Span::raw(num_str));
                if right_pad > 0 {
                    legend_spans.push(Span::raw("─".repeat(right_pad)));
                }
            }

            // Handle remainder/partial segment if exists
            if remainder_width > 0 {
                let num_str = (num_legend_bands + 1).to_string();
                let num_len = num_str.len();

                legend_spans.push(Span::raw("|"));

                if remainder_width == 1 {
                    // Only 1 char: show *
                    legend_spans.push(Span::raw("*".to_string()));
                } else if remainder_width == 2 {
                    // 2 chars: show number directly
                    if num_len == 1 {
                        legend_spans.push(Span::raw(num_str));
                    } else {
                        legend_spans.push(Span::raw("*".to_string()));
                    }
                } else {
                    // 3+ chars: show with padding
                    let padding = remainder_width.saturating_sub(num_len + 1); // 1 for |
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;

                    if left_pad > 0 {
                        legend_spans.push(Span::raw("─".repeat(left_pad)));
                    }
                    legend_spans.push(Span::raw(num_str.clone()));
                    if right_pad > 0 {
                        legend_spans.push(Span::raw("─".repeat(right_pad)));
                    }
                }
            }

            // Add final closing pipe
            legend_spans.push(Span::raw("|"));

            let legend_indicators = Paragraph::new(Line::from(legend_spans))
                .block(Block::default().borders(Borders::ALL).title("Frequency Ranges"));
            f.render_widget(legend_indicators, chunks[1]);

            // Legend details (frequency ranges with colors)
            let mut legend_details = String::new();
            let total_legend_items = if remainder_width > 0 { num_legend_bands + 1 } else { num_legend_bands };

            for i in 0..total_legend_items {
                let band_index = (i * num_bands) / num_legend_bands.max(1);

                // Calculate frequency range for this legend band
                let min_freq: f32 = 20.0;
                let max_freq_val: f32 = (sample_rate / 2) as f32;
                let log_min = min_freq.ln();
                let log_max = max_freq_val.ln();

                let log_start = log_min + (band_index as f32 / num_bands as f32) * (log_max - log_min);
                let freq_start = log_start.exp();

                let freq_label = if freq_start < 1000.0 {
                    format!("{}:{:.0}Hz", i + 1, freq_start)
                } else {
                    format!("{}:{:.1}kHz", i + 1, freq_start / 1000.0)
                };

                legend_details.push_str(&format!("{:12}", freq_label));

                // Add newline every 6 items for better layout
                if (i + 1) % 6 == 0 {
                    legend_details.push('\n');
                }
            }

            let legend_widget = Paragraph::new(legend_details)
                .block(Block::default().borders(Borders::ALL).title("Band Details"));
            f.render_widget(legend_widget, chunks[2]);

            // Time display
            let time_text = format!(
                "Playing: {:.2}s / {:.2}s | Bands: {} | Press 'q' or Ctrl+C to exit",
                elapsed, total_duration, num_bands
            );
            let time_widget = Paragraph::new(time_text)
                .block(Block::default().borders(Borders::ALL).title("Progress"));
            f.render_widget(time_widget, chunks[3]);
        })?;
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the WAV file
    let file = File::open("src/sound4.wav")?;
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
    let file = File::open("src/sound4.wav")?;
    let source = Decoder::new(BufReader::new(file))?;
    let source = rodio::source::UniformSourceIterator::new(source, 1, sample_rate);

    // Wrap source with our sample capture
    let (wrapped_source, sample_buffer) = SampleCapture::new(source, sample_rate);

    // Add audio to sink and play
    sink.append(wrapped_source);

    // Shared flag to signal threads to stop
    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = should_stop.clone();

    // Spawn thread to perform FFT and display
    let handle = std::thread::spawn(move || {
        if let Err(e) = visualize_frequencies(sample_buffer, sample_rate, duration, should_stop_clone) {
            eprintln!("Visualization error: {}", e);
        }
    });

    // Monitor for stop signal while playing
    while !sink.empty() && !should_stop.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Stop audio immediately if requested
    if should_stop.load(Ordering::Relaxed) {
        sink.stop();
    }

    // Wait for visualization thread
    handle.join().unwrap();

    Ok(())
}
