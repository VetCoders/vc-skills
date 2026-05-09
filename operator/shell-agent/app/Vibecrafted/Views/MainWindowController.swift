import AppKit

class MainWindowController: NSWindowController {
    
    init() {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 1200, height: 800),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        window.title = "Vibecrafted"
        window.minSize = NSSize(width: 800, height: 600)
        window.center()
        window.isReleasedWhenClosed = false
        
        super.init(window: window)
        
        self.contentViewController = MainSplitViewController()
        setupToolbar()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupToolbar() {
        let toolbar = NSToolbar(identifier: "MainWindowToolbar")
        toolbar.displayMode = .iconOnly
        toolbar.allowsUserCustomization = false
        // For simplicity, minimal toolbar config here
        self.window?.toolbar = toolbar
    }
}
