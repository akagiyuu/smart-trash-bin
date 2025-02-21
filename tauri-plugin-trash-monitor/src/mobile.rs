use serde::{de::DeserializeOwned, Serialize};
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;
use crate::Result;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_trash_monitor);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<TrashMonitor<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.trash_monitor", "TrashMonitorPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_trash_monitor)?;

    let trash_monitor = TrashMonitor(handle);

    Ok(trash_monitor)
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Payload {}

/// Access to the trash-monitor APIs.
pub struct TrashMonitor<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TrashMonitor<R> {
    pub fn start_monitoring(&self) {
        self.0.run_mobile_plugin::<serde_json::Value>("startMonitoring", ()).unwrap();
    }

    pub fn stop_monitoring(&self) {
        self.0.run_mobile_plugin::<serde_json::Value>("stopMonitoring", ()).unwrap();
    }
}
