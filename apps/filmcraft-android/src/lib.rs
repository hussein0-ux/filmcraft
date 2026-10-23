//! FilmCraft on Android: eframe's NativeActivity runner on wgpu (Vulkan), same engine and UI crates.
//!
//! EXPERIMENTAL scaffold, not yet compiled in CI. Not wired up yet: audio output (cpal/oboe),
//! file pickers (Storage Access Framework), hardware decoding (MediaCodec), control server.
//! Empty on non-Android targets.
#![cfg(target_os = "android")]

use filmcraft_engine::Session;
use filmcraft_engine::autosave::AutosaveConfig;
use filmcraft_ui_egui::FilmcraftApp;
use serde_json::json;

use eframe::egui_wgpu;
use winit_android::AndroidApp;

mod winit_android {
    pub use eframe::egui_winit::winit::platform::android::activity::AndroidApp;
}

#[allow(unsafe_code)] // the JNI/NativeActivity entry symbol must be unmangled
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag("filmcraft"));
    if let Err(e) = run(app) {
        log::error!("FilmCraft could not start: {e}");
    }
}

fn run(android: AndroidApp) -> Result<(), eframe::Error> {
    let data_dir = android.internal_data_path();
    let options = eframe::NativeOptions {
        android_app: Some(android),
        wgpu_options: egui_wgpu::WgpuConfiguration::default(),
        ..Default::default()
    };
    eframe::run_native(
        "FilmCraft",
        options,
        Box::new(move |cc| {
            let mut session = Session::default();
            if let Some(dir) = data_dir.clone() {
                if let Err(e) = session.start_autosave(AutosaveConfig::new(dir)) {
                    log::warn!("auto-save unavailable: {e}");
                }
            }
            let _ = session.execute("file.openDemoProject", json!({}));
            let mut app = FilmcraftApp::new(session);
            if let Some(rs) = cc.wgpu_render_state.clone() {
                app.set_wgpu(rs);
            }
            Ok(Box::new(app))
        }),
    )
}
