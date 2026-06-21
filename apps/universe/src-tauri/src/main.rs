#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tench_ui::run_native_with_config(
        tench_ui::NativeConfig {
            title: "Tench Universe".into(),
            width: 1280.0,
            height: 820.0,
            resizable: true,
        },
        |backend| backend.set_root(tench_universe_lib::ui::UniverseApp::new()),
    );
}
