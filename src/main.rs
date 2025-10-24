use std::fs::File;
use std::io::BufReader;

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

    Ok(())
}
