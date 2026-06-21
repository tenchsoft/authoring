mod commands;
mod markdown_io;
pub mod ui;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Legacy commands
            commands::runtime_capabilities,
            commands::export_to_markdown,
            commands::export_to_html,
            commands::export_to_txt,
            commands::pick_save_path,
            commands::pick_export_path,
            commands::pick_import_path,
            commands::read_text_file,
            commands::write_text_file,
            // Phase 7: File format commands
            commands::save_project_bundle,
            commands::load_project_bundle,
            commands::export_docx,
            commands::export_pdf,
            commands::export_epub,
            commands::export_markdown,
            commands::export_html,
            commands::export_txt,
            commands::import_docx,
            commands::import_markdown,
        ])
        .setup(|app| {
            crate::init_tenchi_ui(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Tench Story");
}

pub type BackendState = tench_ui::platform::TauriBackendState;

/// Initialize tench-ui rendering on a Tauri window.
pub fn init_tenchi_ui(app: &mut tauri::App) {
    tench_ui::platform::init_tauri_ui(
        app,
        tench_ui::platform::TauriUiOptions::default(),
        |backend, _app| {
            backend.set_root(ui::StoryApp::new());
        },
    );
}
