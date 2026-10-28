#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // الميزات المشتركة بين الكمبيوتر والأندرويد
    builder = builder.plugin(tauri_plugin_store::Builder::new().build());

    // الميزات الخاصة بالكمبيوتر فقط (يتم تجاهلها في الأندرويد)
    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_window_state::Builder::default().build())
            .plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
