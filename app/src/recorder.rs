mod pvrecorder;
// mod cpal;
// mod portaudio;

use once_cell::sync::OnceCell;

use crate::{DB, config, config::structs::RecorderType, safe_globals};

static RECORDER_TYPE: OnceCell<RecorderType> = OnceCell::new();
static FRAME_LENGTH: OnceCell<u32> = OnceCell::new();

pub fn init() -> Result<(), ()> {
    // set default recorder type
    // @TODO. Make it configurable?
    if let Err(_) = RECORDER_TYPE.set(config::DEFAULT_RECORDER_TYPE) {
        error!("Recorder type already initialized");
        return Err(());
    }

    // load given recorder
    let recorder_type = match RECORDER_TYPE.get() {
        Some(rt) => rt,
        None => {
            error!("Failed to get recorder type after initialization");
            return Err(());
        }
    };
    
    match recorder_type {
        RecorderType::PvRecorder => {
            // Init Pv Recorder
            info!("Initializing PvRecorder recording backend.");
            if let Err(_) = FRAME_LENGTH.set(512u32) {
                error!("Frame length already initialized");
                return Err(());
            }
            
            let mic_index = match get_selected_microphone_index() {
                Ok(idx) => idx,
                Err(e) => {
                    error!("Failed to get microphone index: {}", e);
                    return Err(());
                }
            };
            
            let frame_len = match FRAME_LENGTH.get() {
                Some(len) => *len,
                None => {
                    error!("Frame length not initialized");
                    return Err(());
                }
            };
            
            match pvrecorder::init_microphone(mic_index, frame_len) {
                false => {
                    error!("Recorder initialization failed.");

                    return Err(())
                },
                _ => {
                    info!("Recorder initialization success.");
                }
            }
        },
        RecorderType::PortAudio => {
            // Init PortAudio
            info!("Initializing PortAudio recording backend");
            error!("PortAudio backend is not implemented yet");
            return Err(());
        },
        RecorderType::Cpal => {
            // Init CPAL
            info!("Initializing CPAL recording backend");
            error!("CPAL backend is not implemented yet");
            return Err(());
        }
    }

    Ok(())
}

pub fn read_microphone(frame_buffer: &mut [i16]) {
    let recorder_type = match RECORDER_TYPE.get() {
        Some(rt) => rt,
        None => {
            error!("Recorder type not initialized");
            frame_buffer.fill(0);
            return;
        }
    };
    
    match recorder_type {
        RecorderType::PvRecorder => {
            pvrecorder::read_microphone(frame_buffer);
        },
        RecorderType::PortAudio => {
            error!("PortAudio backend is not implemented");
            // Fill buffer with silence to avoid undefined behavior
            frame_buffer.fill(0);
        },
        RecorderType::Cpal => {
            // CPAL uses callback-based audio processing and shouldn't be used with blocking reads
            error!("CPAL recorder doesn't support blocking read operations - use callback-based processing instead");
            // Fill buffer with silence to avoid undefined behavior
            frame_buffer.fill(0);
        }
    }
}

pub fn start_recording() -> Result<(), ()> {
    let recorder_type = match RECORDER_TYPE.get() {
        Some(rt) => rt,
        None => {
            error!("Recorder type not initialized");
            return Err(());
        }
    };
    
    match recorder_type {
        RecorderType::PvRecorder => {
            let mic_index = match get_selected_microphone_index() {
                Ok(idx) => idx,
                Err(e) => {
                    error!("Failed to get microphone index: {}", e);
                    return Err(());
                }
            };
            
            let frame_len = match FRAME_LENGTH.get() {
                Some(len) => *len,
                None => {
                    error!("Frame length not initialized");
                    return Err(());
                }
            };
            
            return pvrecorder::start_recording(mic_index, frame_len);
        },
        RecorderType::PortAudio => {
            error!("PortAudio backend is not implemented");
            return Err(());
        },
        RecorderType::Cpal => {
            error!("CPAL backend is not implemented");
            return Err(());
        }
    }
}

pub fn stop_recording() -> Result<(), ()> {
    let recorder_type = match RECORDER_TYPE.get() {
        Some(rt) => rt,
        None => {
            error!("Recorder type not initialized");
            return Err(());
        }
    };
    
    match recorder_type {
        RecorderType::PvRecorder => {
            pvrecorder::stop_recording()
        },
        RecorderType::PortAudio => {
            error!("PortAudio backend is not implemented");
            return Err(());
        },
        RecorderType::Cpal => {
            error!("CPAL backend is not implemented");
            return Err(());
        }
    }
}

pub fn get_selected_microphone_index() -> Result<i32, &'static str> {
    match safe_globals::get_db() {
        Ok(db) => Ok(db.microphone),
        Err(e) => Err(e),
    }
}