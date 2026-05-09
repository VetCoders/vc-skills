// mudd.one — Main Split View (sidebar | canvas | inspector)
// Created by M&K (c)2026 VetCoders

import AppKit

class MainSplitViewController: NSSplitViewController {
    let sidebarVC = SidebarViewController()
    let canvasVC = CanvasViewController()
    let inspectorVC = InspectorViewController()

    override func viewDidLoad() {
        super.viewDidLoad()

        let sidebarItem = NSSplitViewItem(sidebarWithViewController: sidebarVC)
        sidebarItem.minimumThickness = 180
        sidebarItem.maximumThickness = 300
        sidebarItem.canCollapse = true

        let canvasItem = NSSplitViewItem(viewController: canvasVC)
        canvasItem.minimumThickness = 400

        let inspectorItem = NSSplitViewItem(inspectorWithViewController: inspectorVC)
        inspectorItem.minimumThickness = 200
        inspectorItem.maximumThickness = 350

        addSplitViewItem(sidebarItem)
        addSplitViewItem(canvasItem)
        addSplitViewItem(inspectorItem)
    }
}
