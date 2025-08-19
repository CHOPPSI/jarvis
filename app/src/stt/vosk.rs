use once_cell::sync::OnceCell;
use vosk::{DecodingState, Model, Recognizer};

use std::sync::Mutex;

use crate::config::VOSK_MODEL_PATH;

static MODEL: OnceCell<Model> = OnceCell::new();
static RECOGNIZER: OnceCell<Mutex<Recognizer>> = OnceCell::new();

pub fn init_vosk() {
    if !RECOGNIZER.get().is_none() {return;} // already initialized

    let model = Model::new(VOSK_MODEL_PATH).unwrap();
    let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    MODEL.set(model);
    RECOGNIZER.set(Mutex::new(recognizer));
}

pub fn recognize(data: &[i16], include_partial: bool) -> Option<String> {
    // Safe access to recognizer with single lock acquisition
    let recognizer = match RECOGNIZER.get() {
        Some(r) => r,
        None => {
            error!("VOSK recognizer not initialized");
            return None;
        }
    };

    let mut guard = match recognizer.lock() {
        Ok(guard) => guard,
        Err(e) => {
            error!("Failed to lock VOSK recognizer: {}", e);
            return None;
        }
    };

    let state = guard.accept_waveform(data);

    match state {
        DecodingState::Running => {
            if include_partial {
                Some(guard.partial_result().partial.into())
            } else {
                None
            }
        }
        DecodingState::Finalized => {
            // Result will always be multiple because we called set_max_alternatives
            match guard.result().multiple() {
                Some(result) => {
                    result.alternatives
                        .first()
                        .map(|alt| alt.text.clone())
                }
                None => {
                    warn!("VOSK returned finalized state but no alternatives");
                    None
                }
            }
        }
        DecodingState::Failed => {
            warn!("VOSK decoding failed");
            None
        }
    }
}

// pub fn stereo_to_mono(input_data: &[i16]) -> Vec<i16> {
//     let mut result = Vec::with_capacity(input_data.len() / 2);
//     result.extend(
//         input_data
//             .chunks_exact(2)
//             .map(|chunk| chunk[0] / 2 + chunk[1] / 2),
//     );

//     result
// }
