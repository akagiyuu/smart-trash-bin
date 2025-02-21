package com.plugin.trash_monitor

import android.app.Activity
import android.content.Intent
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class TrashMonitorPlugin(private val activity: Activity) : Plugin(activity) {

    companion object {
        const val TAG = "TrashMonitorPlugin"
    }

    @Command
    fun startMonitoring(invoke: Invoke) {
        Log.i(TAG, "startMonitoring called")
        val serviceIntent = Intent(activity, TrashMonitorService::class.java)
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
            Log.i(TAG, "Starting foreground service")
            activity.startForegroundService(serviceIntent)
        } else {
            Log.i(TAG, "Starting service normally")
            activity.startService(serviceIntent)
        }
        Log.i(TAG, "Service start command issued")
        invoke.resolve(null)
    }

    @Command
    fun stopMonitoring(invoke: Invoke) {
        Log.i(TAG, "stopMonitoring called")
        val serviceIntent = Intent(activity, TrashMonitorService::class.java)
        activity.stopService(serviceIntent)
        Log.i(TAG, "Service stop command issued")
        invoke.resolve(null)
    }
}
