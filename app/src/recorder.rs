mod pvrecorder;
// mod cpal;
// mod portaudio;

use once_cell::sync::OnceCell;

use crate::{DB, config, config::structs::RecorderType};

static RECORDER_TYPE: OnceCell<RecorderType> = OnceCell::new();
static FRAME_LENGTH: OnceCell<u32> = OnceCell::new();

pub fn init() -> Result<(), ()> {
    // set default recorder type
    // @TODO. Make it configurable?
    RECORDER_TYPE.set(config::DEFAULT_RECORDER_TYPE).unwrap();

    // load given recorder
    match RECORDER_TYPE.get().unwrap() {
        RecorderType::PvRecorder => {
            // Init Pv Recorder
            info!("Initializing PvRecorder recording backend.");
            FRAME_LENGTH.set(512u32).unwrap(); // pvrecorder requires frame buffer of 512
            match pvrecorder::init_microphone(get_selected_microphone_index(), FRAME_LENGTH.get().unwrap().to_owned()) {
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
    match RECORDER_TYPE.get().unwrap() {
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
    match RECORDER_TYPE.get().unwrap() {
        RecorderType::PvRecorder => {
            return pvrecorder::start_recording(get_selected_microphone_index(), FRAME_LENGTH.get().unwrap().to_owned());
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
    match RECORDER_TYPE.get().unwrap() {
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

pub fn get_selected_microphone_index() -> i32 {
    DB.get().unwrap().microphone
}