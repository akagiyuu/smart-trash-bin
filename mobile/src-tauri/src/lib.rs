use tauri_plugin_notification::{NotificationExt, PermissionState};
use tauri_plugin_trash_monitor::TrashMonitorExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_trash_monitor::init())
        .setup(|app| {
            let notification = app.notification();

            while notification.permission_state()? != PermissionState::Granted {
                notification.request_permission()?;
            }


            app.trash_monitor().start_monitoring();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
