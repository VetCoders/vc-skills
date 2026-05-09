// mudd.one — Inspector panel (right sidebar: metadata + filters)
// Created by M&K (c)2026 VetCoders

import AppKit

class InspectorViewController: NSViewController {
    private let stackView = NSStackView()
    private let titleLabel = NSTextField(labelWithString: "Inspector")
    private let infoLabel = NSTextField(wrappingLabelWithString: "Open a file to see details")

    // Filters
    private let filterLabel = NSTextField(labelWithString: "Filters")
    private let filterSeparator = NSBox()
    private var filterButtons: [(button: NSButton, filter: FfiFilterType)] = []
    private let applyFiltersButton = NSButton(title: "Apply Filters", target: nil, action: nil)
    private let resetFiltersButton = NSButton(title: "Reset", target: nil, action: nil)

    private var originalFrames: [FfiFrame] = []
    private var currentFrames: [FfiFrame] = []
    private var currentIndex: Int = 0
    private var sessionId: UInt64 = 0

    override func loadView() {
        let container = NSView()
        container.wantsLayer = true
        view = container

        titleLabel.font = .boldSystemFont(ofSize: 14)
        titleLabel.alignment = .center

        infoLabel.font = .monospacedSystemFont(ofSize: 10, weight: .regular)
        infoLabel.textColor = .secondaryLabelColor

        // Filter section
        filterSeparator.boxType = .separator
        filterLabel.font = .boldSystemFont(ofSize: 11)
        filterLabel.textColor = .secondaryLabelColor

        let filters: [(String, FfiFilterType)] = [
            ("Histogram Eq", .histogramEqualization),
            ("Contrast", .contrastStretch),
            ("Adaptive Threshold", .adaptiveThreshold),
            ("Canny Edge", .canny),
            ("Gaussian Blur", .gaussianBlur),
        ]

        for (name, filterType) in filters {
            let btn = NSButton(checkboxWithTitle: name, target: self, action: #selector(filterToggled))
            btn.controlSize = .small
            btn.font = .systemFont(ofSize: 11)
            btn.isEnabled = false
            filterButtons.append((btn, filterType))
        }

        applyFiltersButton.bezelStyle = .accessoryBarAction
        applyFiltersButton.target = self
        applyFiltersButton.action = #selector(applySelectedFilters)
        applyFiltersButton.isEnabled = false

        resetFiltersButton.bezelStyle = .accessoryBarAction
        resetFiltersButton.target = self
        resetFiltersButton.action = #selector(resetFilters)
        resetFiltersButton.isEnabled = false

        // Layout
        stackView.orientation = .vertical
        stackView.alignment = .leading
        stackView.spacing = 8
        stackView.edgeInsets = NSEdgeInsets(top: 12, left: 12, bottom: 12, right: 12)
        stackView.translatesAutoresizingMaskIntoConstraints = false

        stackView.addArrangedSubview(titleLabel)
        stackView.addArrangedSubview(infoLabel)
        stackView.addArrangedSubview(filterSeparator)
        stackView.addArrangedSubview(filterLabel)

        for (btn, _) in filterButtons {
            stackView.addArrangedSubview(btn)
        }

        let buttonRow = NSStackView()
        buttonRow.orientation = .horizontal
        buttonRow.spacing = 8
        buttonRow.addArrangedSubview(applyFiltersButton)
        buttonRow.addArrangedSubview(resetFiltersButton)
        stackView.addArrangedSubview(buttonRow)

        let spacer = NSView()
        spacer.setContentHuggingPriority(.defaultLow, for: .vertical)
        stackView.addArrangedSubview(spacer)

        container.addSubview(stackView)
        NSLayoutConstraint.activate([
            stackView.topAnchor.constraint(equalTo: container.topAnchor),
            stackView.leadingAnchor.constraint(equalTo: container.leadingAnchor),
            stackView.trailingAnchor.constraint(equalTo: container.trailingAnchor),
            stackView.bottomAnchor.constraint(equalTo: container.bottomAnchor),
        ])

        NotificationCenter.default.addObserver(
            self, selector: #selector(handleFramesLoaded),
            name: .muddFramesLoaded, object: nil
        )
        NotificationCenter.default.addObserver(
            self, selector: #selector(handleFrameUpdated),
            name: .muddFrameUpdated, object: nil
        )
        NotificationCenter.default.addObserver(
            self, selector: #selector(handleIndexChanged),
            name: .muddCurrentIndexChanged, object: nil
        )
    }

    // MARK: - Notifications

    @objc private func handleFramesLoaded(_ notification: Notification) {
        guard let frames = notification.userInfo?["frames"] as? [FfiFrame],
              let first = frames.first else { return }

        sessionId += 1
        originalFrames = frames
        currentFrames = frames
        currentIndex = 0
        updateInfoLabel(first, count: frames.count)
        enableFilterButtons(true)
    }

    @objc private func handleFrameUpdated(_ notification: Notification) {
        guard let frame = notification.userInfo?["frame"] as? FfiFrame,
              let index = notification.userInfo?["index"] as? Int else { return }

        let source = notification.userInfo?["source"] as? String ?? "crop"

        if index < currentFrames.count {
            currentFrames[index] = frame
            // Only overwrite original for destructive ops (crop), not filters
            if source == "crop" {
                originalFrames[index] = frame
                for (btn, _) in filterButtons {
                    btn.state = .off
                }
            }
        }
        updateInfoLabel(frame, count: currentFrames.count)
    }

    private func updateInfoLabel(_ frame: FfiFrame, count: Int) {
        let colorspace: String
        switch frame.channels {
        case 1: colorspace = "Grayscale"
        case 3: colorspace = "RGB"
        case 4: colorspace = "RGBA"
        default: colorspace = "\(frame.channels) channels"
        }

        let dataSize = currentFrames.reduce(0) { $0 + $1.data.count }
        let dataMB = String(format: "%.1f", Double(dataSize) / 1_048_576.0)

        infoLabel.stringValue = """
        Dimensions: \(frame.width) x \(frame.height)
        Color space: \(colorspace)
        Frames: \(count)
        Data size: \(dataMB) MB
        Stride: \(frame.width * UInt32(frame.channels)) bytes/row
        """
    }

    private func enableFilterButtons(_ enabled: Bool) {
        for (btn, _) in filterButtons {
            btn.isEnabled = enabled
        }
        applyFiltersButton.isEnabled = enabled
        resetFiltersButton.isEnabled = enabled
    }

    @objc private func handleIndexChanged(_ notification: Notification) {
        guard let index = notification.userInfo?["index"] as? Int else { return }
        currentIndex = index
        // Reset filter checkboxes when switching frames
        for (btn, _) in filterButtons {
            btn.state = .off
        }
        if index < currentFrames.count {
            updateInfoLabel(currentFrames[index], count: currentFrames.count)
        }
    }

    // MARK: - Filters

    @objc private func filterToggled() {
        // Visual feedback only — actual apply on button press
    }

    @objc private func applySelectedFilters() {
        let selected = filterButtons.compactMap { $0.button.state == .on ? $0.filter : nil }
        guard !selected.isEmpty, !originalFrames.isEmpty else { return }

        let idx = currentIndex
        let session = sessionId
        let frame = originalFrames[idx]
        applyFiltersButton.isEnabled = false
        applyFiltersButton.title = "Applying..."

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            do {
                let result = try applyFilters(frame: frame, filterTypes: selected)
                DispatchQueue.main.async {
                    guard self?.sessionId == session else { return }
                    self?.applyFiltersButton.title = "Apply Filters"
                    self?.applyFiltersButton.isEnabled = true
                    if idx < (self?.currentFrames.count ?? 0) {
                        self?.currentFrames[idx] = result
                    }
                    NotificationCenter.default.post(
                        name: .muddFrameUpdated, object: self,
                        userInfo: ["frame": result, "index": idx, "source": "filter"]
                    )
                }
            } catch {
                DispatchQueue.main.async {
                    guard self?.sessionId == session else { return }
                    self?.applyFiltersButton.title = "Apply Filters"
                    self?.applyFiltersButton.isEnabled = true
                    let alert = NSAlert()
                    alert.messageText = "Filter Failed"
                    alert.informativeText = error.localizedDescription
                    alert.runModal()
                }
            }
        }
    }

    @objc private func resetFilters() {
        guard !originalFrames.isEmpty else { return }
        for (btn, _) in filterButtons {
            btn.state = .off
        }
        let original = originalFrames[currentIndex]
        currentFrames[currentIndex] = original
        NotificationCenter.default.post(
            name: .muddFrameUpdated, object: self,
            userInfo: ["frame": original, "index": currentIndex, "source": "filter"]
        )
    }
}
