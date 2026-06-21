#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tench_ui::run_native_with_config(
        tench_ui::NativeConfig {
            title: "Tench Story".into(),
            width: 1360.0,
            height: 860.0,
            resizable: true,
        },
        |backend| backend.set_root(tench_story_lib::ui::StoryApp::new()),
    );
}
