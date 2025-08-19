use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

#[tauri::command(async)]
pub fn play_sound(filename: &str, sleep: bool) -> Result<(), String> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| format!("Failed to get audio output stream: {}", e))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| format!("Failed to create audio sink: {}", e))?;
    
    // Load a sound from a file, using a path relative to Cargo.toml
    // let filepath = format!("{PUBLIC_PATH}/sound/{filename}.wav");
    let filepath = filename;
    let file = File::open(&filepath)
        .map_err(|e| format!("Failed to open audio file '{}': {}", filepath, e))?;
    let file = BufReader::new(file);
    
    // Decode that sound file into a source
    let source = Decoder::new(file)
        .map_err(|e| format!("Failed to decode audio file '{}': {}", filepath, e))?;
    
    // Play the sound directly on the device
    println!("Playing {} ...", filepath);
    // stream_handle.play_raw(source.convert_samples());
    sink.append(source);

    if sleep {
        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
    
    Ok(())
}