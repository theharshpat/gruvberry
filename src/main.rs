use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStreamBuilder, Sink};

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
    let file = File::open("src/sound.wav")?;
    let source = Decoder::new(BufReader::new(file))?;

    println!("\nPlaying wav file... sound.wav");

    // Add audio to sink and play
    sink.append(source);

    // Wait until playback finishes
    sink.sleep_until_end();

    println!("Wav file playing completed");

    Ok(())
}
