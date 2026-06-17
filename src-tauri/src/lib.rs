mod commands;

use commands::{
    activate_provider, get_session_last_message, get_settings_path, list_providers, list_sessions, load_annotations,
    read_model_config, read_session, save_annotation, save_providers, test_connection, write_model_config,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            read_model_config,
            write_model_config,
            get_settings_path,
            list_providers,
            save_providers,
            activate_provider,
            test_connection,
            list_sessions,
            read_session,
            load_annotations,
            save_annotation,
            get_session_last_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
