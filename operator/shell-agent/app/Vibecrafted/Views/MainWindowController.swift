// mudd.one — Main Window Controller
// Created by M&K (c)2026 VetCoders

import AppKit

class MainWindowController: NSWindowController, NSToolbarDelegate {
    private let mainViewController = MainSplitViewController()

    private let toolbarOpenItem = NSToolbarItem.Identifier("openFile")
    private let toolbarSidebarItem = NSToolbarItem.Identifier("toggleSidebar")
    private let toolbarInspectorItem = NSToolbarItem.Identifier("toggleInspector")

    init() {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 1200, height: 800),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        window.title = "mudd.one"
        window.titleVisibility = .hidden
        window.titlebarAppearsTransparent = true
        window.toolbarStyle = .unified
        window.center()
        window.setFrameAutosaveName("MuddMainWindow")
        window.contentViewController = mainViewController
        window.minSize = NSSize(width: 800, height: 600)

        super.init(window: window)

        // Toolbar — after super.init so `self` is valid
        let toolbar = NSToolbar(identifier: "MuddToolbar")
        toolbar.delegate = self
        toolbar.displayMode = .iconOnly
        window.toolbar = toolbar
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        fatalError()
    }

    // MARK: - Open File (responder chain target)

    @objc func openFile(_ sender: Any?) {
        guard let window = window else { return }
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [
            .init(filenameExtension: "dcm")!,
            .init(filenameExtension: "dicom")!,
            .png, .jpeg, .tiff, .bmp,
            .init(filenameExtension: "mp4")!,
            .init(filenameExtension: "avi")!,
            .init(filenameExtension: "mov")!,
        ]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        panel.message = "Select a DICOM, image, or video file"

        panel.beginSheetModal(for: window) { response in
            guard response == .OK, let url = panel.url else { return }
            NotificationCenter.default.post(
                name: .muddFileSelected, object: nil,
                userInfo: ["url": url]
            )
        }
    }

    // MARK: - NSToolbarDelegate

    func toolbar(_ toolbar: NSToolbar, itemForItemIdentifier itemIdentifier: NSToolbarItem.Identifier, willBeInsertedIntoToolbar flag: Bool) -> NSToolbarItem? {
        switch itemIdentifier {
        case toolbarOpenItem:
            let item = NSToolbarItem(itemIdentifier: itemIdentifier)
            item.label = "Open"
            item.toolTip = "Open File (Cmd+O)"
            item.image = NSImage(systemSymbolName: "doc.badge.plus", accessibilityDescription: "Open File")
            item.target = nil
            item.action = #selector(openFile(_:))
            return item

        case toolbarSidebarItem:
            let item = NSToolbarItem(itemIdentifier: itemIdentifier)
            item.label = "Sidebar"
            item.toolTip = "Toggle Sidebar"
            item.image = NSImage(systemSymbolName: "sidebar.left", accessibilityDescription: "Toggle Sidebar")
            item.target = mainViewController
            item.action = #selector(NSSplitViewController.toggleSidebar(_:))
            return item

        case toolbarInspectorItem:
            let item = NSToolbarItem(itemIdentifier: itemIdentifier)
            item.label = "Inspector"
            item.toolTip = "Toggle Inspector"
            item.image = NSImage(systemSymbolName: "sidebar.right", accessibilityDescription: "Toggle Inspector")
            item.target = mainViewController
            item.action = #selector(NSSplitViewController.toggleInspector(_:))
            return item

        default:
            return nil
        }
    }

    func toolbarDefaultItemIdentifiers(_ toolbar: NSToolbar) -> [NSToolbarItem.Identifier] {
        [toolbarSidebarItem, toolbarOpenItem, .flexibleSpace, toolbarInspectorItem]
    }

    func toolbarAllowedItemIdentifiers(_ toolbar: NSToolbar) -> [NSToolbarItem.Identifier] {
        [toolbarSidebarItem, toolbarOpenItem, .flexibleSpace, toolbarInspectorItem]
    }
}
