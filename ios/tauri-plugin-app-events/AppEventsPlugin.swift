//
//  AppEventsPlugin.swift
//  tauri-plugin-app-events
//
//  Created by wtto on 2024/10/28.
//

import SwiftRs
import Tauri
import UIKit
import WebKit
import OSLog

let log = OSLog(subsystem: "com.tauri.dev", category: "plugin.app.events")

class AppEvetnsPlugin: Plugin {
  override init() {
    super.init()
    NotificationCenter.default.addObserver(self,selector: #selector(applicationDidBecomeActive),name: UIApplication.didBecomeActiveNotification,object: nil)
    NotificationCenter.default.addObserver(self, selector: #selector(applicationDidEnterBackground), name: UIApplication.didEnterBackgroundNotification, object: nil)
  }
  
  @objc func applicationDidBecomeActive(notification: NSNotification) {
    os_log(.debug, log: log, "Application Did Become Active")
    trigger("resume", data: JSObject())
  }
  
  @objc func applicationDidEnterBackground(notification: NSNotification) {
    os_log(.debug, log: log, "Application Did Enter Background")
    trigger("pause", data: JSObject())
  }
}

@_cdecl("init_plugin_app_events")
func initPlugin() -> Plugin {
  return AppEvetnsPlugin()
}
