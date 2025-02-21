package com.plugin.trash_monitor

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.core.app.NotificationCompat
import okhttp3.*
import org.json.JSONObject
import java.util.concurrent.TimeUnit

class TrashMonitorService : Service() {

    companion object {
        const val CHANNEL_ID = "trash_monitor_channel"
        const val TAG = "TrashMonitorService"
    }

    private lateinit var webSocket: WebSocket
    private val client: OkHttpClient = OkHttpClient.Builder()
        .readTimeout(0, TimeUnit.MILLISECONDS)
        .build()

    override fun onCreate() {
        super.onCreate()
        Log.i(TAG, "Service onCreate")
        createNotificationChannel()
        val notification = createNotification("Monitoring trash level...")
        startForeground(1, notification)
        startWebSocketConnection()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.i(TAG, "Service onStartCommand: startId = $startId")
        return START_STICKY
    }

    override fun onDestroy() {
        Log.i(TAG, "Service onDestroy")
        webSocket.cancel()
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? {
        // We don't support binding
        return null
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Log.i(TAG, "Creating notification channel")
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Trash Monitor Channel",
                NotificationManager.IMPORTANCE_LOW
            )
            (getSystemService(NotificationManager::class.java)).createNotificationChannel(channel)
        }
    }

    private fun createNotification(content: String): Notification {
        Log.i(TAG, "Creating notification with content: $content")
        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("Trash Monitor")
            .setContentText(content)
            .setSmallIcon(R.drawable.ic_notification)
            .build()
    }

    private fun startWebSocketConnection() {
        Log.i(TAG, "Starting WebSocket connection")
        val request = Request.Builder()
            .url("wss://trashcan-api.arisavinh.dev/device/3fa85f64-5717-4562-b3fc-2c963f66afa6/data")
            .build()
        webSocket = client.newWebSocket(request, object : WebSocketListener() {
            override fun onOpen(webSocket: WebSocket, response: Response) {
                Log.i(TAG, "WebSocket onOpen")
            }

            override fun onMessage(webSocket: WebSocket, text: String) {
                Log.d(TAG, "Received WebSocket message: $text")
                try {
                    val json = JSONObject(text)
                    val trashLevel = json.optDouble("trash_level", 0.0)
                    Log.i(TAG, "Parsed trash_level: $trashLevel")
                    if (trashLevel > 80.0) {
                        showAlertNotification("Trash Alert", "Trash level is $trashLevel%!")
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "Error parsing WebSocket message", e)
                }
            }

            override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                Log.e(TAG, "WebSocket onFailure", t)
            }
        })
    }

    private fun showAlertNotification(title: String, content: String) {
        Log.i(TAG, "Showing alert notification: $title - $content")
        val notification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle(title)
            .setContentText(content)
            .setSmallIcon(R.drawable.ic_notification)
            .build()
        (getSystemService(NotificationManager::class.java)).notify(2, notification)
    }
}
