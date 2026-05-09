import AppKit
import Foundation

@main
class AppDelegate: NSObject, NSApplicationDelegate, EventCallback {
    var mainWindowController: MainWindowController?
    var hasActiveWindow: Bool = false
    
    // Server status from IPC
    var serverStatus: [FfiMuxService] = []

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        NSApp.setActivationPolicy(.regular)
        showMainWindowIfNeeded()
        
        do {
            // Try to initialize IPC runtime (assuming mux daemon uses default socket path or we pass it)
            // Wait, the plan says "init_runtime(socket_path: String)"
            let socketPath = "/tmp/vibecrafted-mux.sock" // TODO: Use dynamic path if needed
            try initRuntime(socketPath: socketPath)
            
            // Subscribe to events
            try subscribeEvents(callback: self)
            
        } catch {
            print("Failed to initialize IPC: \(error)")
        }
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Teardown
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
    
    func applicationDidBecomeActive(_ notification: Notification) {
        showMainWindowIfNeeded()
    }
    
    func showMainWindowIfNeeded() {
        if !hasActiveWindow {
            if mainWindowController == nil {
                mainWindowController = MainWindowController(window: nil)
            }
            mainWindowController?.showWindow(self)
            mainWindowController?.window?.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
            hasActiveWindow = true
        }
    }
    
    // MARK: - EventCallback
    
    func onEvent(event: FfiIpcEvent) {
        DispatchQueue.main.async {
            switch event {
            case .stateChange(let status, let services):
                self.serverStatus = services
                NotificationCenter.default.post(name: NSNotification.Name("MuxServicesUpdated"), object: nil, userInfo: ["services": services])
            case .serverHealth(let services, _):
                self.serverStatus = services
                NotificationCenter.default.post(name: NSNotification.Name("MuxServicesUpdated"), object: nil, userInfo: ["services": services])
            }
        }
    }
    
    func onError(message: String) {
        print("IPC Error: \(message)")
    }
}
