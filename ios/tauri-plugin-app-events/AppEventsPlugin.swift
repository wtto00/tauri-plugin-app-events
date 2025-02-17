//
//  AppEventsPlugin.swift
//  tauri-plugin-app-events
//
//  Created by wtto on 2024/10/28.
//

import OSLog
import SwiftRs
import Tauri
import UIKit
import WebKit

let log = OSLog(subsystem: "com.tauri.dev", category: "plugin.app.events")

class SetEventHandlerArgs: Decodable {
  let handler: Channel
}

class AppEvetnsPlugin: Plugin {
  private var resumeChannel: Channel? = nil
  private var pauseChannel: Channel? = nil

  override init() {
    super.init()
    NotificationCenter.default.addObserver(self, selector: #selector(applicationDidBecomeActive), name: UIApplication.didBecomeActiveNotification, object: nil)
    NotificationCenter.default.addObserver(self, selector: #selector(applicationDidEnterBackground), name: UIApplication.didEnterBackgroundNotification, object: nil)
  }

  @objc func applicationDidBecomeActive(notification: NSNotification) {
    os_log(.debug, log: log, "Application Did Become Active")
    trigger("resume", data: JSObject())
    resumeChannel?.send(JSObject())
  }

  @objc func applicationDidEnterBackground(notification: NSNotification) {
    os_log(.debug, log: log, "Application Did Enter Background")
    trigger("pause", data: JSObject())
    pauseChannel?.send(JSObject())
  }

  @objc public func setResumeHandler(_ invoke: Invoke) throws {
    os_log(.debug, log: log, "setResumeHandler")
    let args = try invoke.parseArgs(SetEventHandlerArgs.self)
    resumeChannel = args.handler
    invoke.resolve()
  }

  @objc public func setPauseHandler(_ invoke: Invoke) throws {
    os_log(.debug, log: log, "setPauseHandler")
    let args = try invoke.parseArgs(SetEventHandlerArgs.self)
    pauseChannel = args.handler
    invoke.resolve()
  }
}

@_cdecl("init_plugin_app_events")
func initPlugin() -> Plugin {
  return AppEvetnsPlugin()
}
