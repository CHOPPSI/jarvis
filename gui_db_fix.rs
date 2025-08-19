// Исправления для GUI DB проблем

// В файле /workspace/gui/src-tauri/src/tauri_commands/listener.rs
// Заменить строку 60:
// if let Some(wwengine) = DB.lock().unwrap().get::<String>("selected_wake_word_engine") {
// НА:
// match DB.get() {
//     Some(db) => db.wake_word_engine,
//     None => config::DEFAULT_WAKE_WORD_ENGINE
// }

// Заменить строку 421:
// if let Some(pkey) = DB.lock().unwrap().get::<String>("api_key__picovoice") {
// НА:
// match DB.get() {
//     Some(db) => {
//         if db.api_keys.picovoice.is_empty() {
//             return Err("Picovoice API key is not set!".into());
//         }
//         db.api_keys.picovoice.clone()
//     },
//     None => return Err("Database not initialized".into())
// }

// Основная проблема: GUI использует Settings структуру, но код пытается использовать PickleDb методы
// Это критическая архитектурная ошибка