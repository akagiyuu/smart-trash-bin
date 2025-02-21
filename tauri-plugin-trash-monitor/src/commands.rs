use tauri::{command, AppHandle, Runtime};

use crate::TrashMonitorExt;

#[command]
pub(crate) async fn start_monitoring<R: Runtime>(app: AppHandle<R>) {
    app.trash_monitor().start_monitoring();
}

#[command]
pub(crate) async fn stop_monitoring<R: Runtime>(app: AppHandle<R>) {
    app.trash_monitor().stop_monitoring();
}
