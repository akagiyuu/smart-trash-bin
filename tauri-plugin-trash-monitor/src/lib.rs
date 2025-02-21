use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

// #[cfg(desktop)]
// use desktop::TrashMonitor;
use mobile::TrashMonitor;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the trash-monitor APIs.
pub trait TrashMonitorExt<R: Runtime> {
  fn trash_monitor(&self) -> &TrashMonitor<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TrashMonitorExt<R> for T {
  fn trash_monitor(&self) -> &TrashMonitor<R> {
    self.state::<TrashMonitor<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("trash-monitor")
    .invoke_handler(tauri::generate_handler![commands::stop_monitoring, commands::start_monitoring])
    .setup(|app, api| {
      // #[cfg(mobile)]
      let trash_monitor = mobile::init(app, api)?;
      // #[cfg(desktop)]
      // let trash_monitor = desktop::init(app, api)?;
      app.manage(trash_monitor);
      Ok(())
    })
    .build()
}
