// mudd.one — Canvas (image display + ROI overlay + sequence navigator)
// Created by M&K (c)2026 VetCoders

import AppKit

class CanvasViewController: NSViewController {
    private let imageView = NSImageView()
    private let statusLabel = NSTextField(labelWithString: "No file loaded")
    private let sequenceSlider = NSSlider(value: 0, minValue: 0, maxValue: 0, target: nil, action: nil)
    private let frameLabel = NSTextField(labelWithString: "")

    // ROI overlay
    private let roiLayer = CAShapeLayer()
    private var dragStart: NSPoint?
    private var isDragging = false
    private var currentRoi: FfiRoi?

    // Segmentation overlay
    private let maskLayer = CALayer()
    private var segModeActive = false

    private var frames: [FfiFrame] = []
    private var currentIndex: Int = 0
    private var sessionId: UInt64 = 0

    override func loadView() {
        let container = NSView()
        container.wantsLayer = true
        view = container

        // Image view
        imageView.imageScaling = .scaleProportionallyUpOrDown
        imageView.imageAlignment = .alignCenter
        imageView.wantsLayer = true
        imageView.layer?.backgroundColor = NSColor.controlBackgroundColor.cgColor
        imageView.translatesAutoresizingMaskIntoConstraints = false

        // ROI overlay layer
        roiLayer.fillColor = NSColor.systemYellow.withAlphaComponent(0.15).cgColor
        roiLayer.strokeColor = NSColor.systemYellow.cgColor
        roiLayer.lineWidth = 2.0
        roiLayer.lineDashPattern = [6, 3]

        // Mask overlay layer
        maskLayer.opacity = 0.4

        // Status bar
        statusLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        statusLabel.textColor = .secondaryLabelColor
        statusLabel.translatesAutoresizingMaskIntoConstraints = false

        // Sequence navigator
        sequenceSlider.controlSize = .small
        sequenceSlider.target = self
        sequenceSlider.action = #selector(sliderChanged)
        sequenceSlider.isHidden = true
        sequenceSlider.translatesAutoresizingMaskIntoConstraints = false

        frameLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        frameLabel.textColor = .secondaryLabelColor
        frameLabel.alignment = .center
        frameLabel.isHidden = true
        frameLabel.translatesAutoresizingMaskIntoConstraints = false

        container.addSubview(statusLabel)
        container.addSubview(imageView)
        container.addSubview(sequenceSlider)
        container.addSubview(frameLabel)

        NSLayoutConstraint.activate([
            statusLabel.topAnchor.constraint(equalTo: container.topAnchor, constant: 4),
            statusLabel.leadingAnchor.constraint(equalTo: container.leadingAnchor, constant: 8),
            statusLabel.trailingAnchor.constraint(equalTo: container.trailingAnchor, constant: -8),

            imageView.topAnchor.constraint(equalTo: statusLabel.bottomAnchor, constant: 4),
            imageView.leadingAnchor.constraint(equalTo: container.leadingAnchor),
            imageView.trailingAnchor.constraint(equalTo: container.trailingAnchor),
            imageView.bottomAnchor.constraint(equalTo: sequenceSlider.topAnchor, constant: -4),

            sequenceSlider.leadingAnchor.constraint(equalTo: container.leadingAnchor, constant: 8),
            sequenceSlider.trailingAnchor.constraint(equalTo: container.trailingAnchor, constant: -8),
            sequenceSlider.bottomAnchor.constraint(equalTo: frameLabel.topAnchor, constant: -2),

            frameLabel.leadingAnchor.constraint(equalTo: container.leadingAnchor),
            frameLabel.trailingAnchor.constraint(equalTo: container.trailingAnchor),
            frameLabel.bottomAnchor.constraint(equalTo: container.bottomAnchor, constant: -4),
            frameLabel.heightAnchor.constraint(equalToConstant: 16),
        ])

        NotificationCenter.default.addObserver(
            self, selector: #selector(handleFileSelected),
            name: .muddFileSelected, object: nil
        )
        NotificationCenter.default.addObserver(
            self, selector: #selector(handleRoiDetected),
            name: .muddRoiDetected, object: nil
        )
        NotificationCenter.default.addObserver(
            self, selector: #selector(handleFrameUpdated),
            name: .muddFrameUpdated, object: nil
        )
        NotificationCenter.default.addObserver(
            self, selector: #selector(handleSegModeChanged),
            name: Notification.Name("muddSegModeChanged"), object: nil
        )
    }

    override func viewDidLayout() {
        super.viewDidLayout()
        if roiLayer.superlayer == nil {
            imageView.layer?.addSublayer(roiLayer)
        }
        if maskLayer.superlayer == nil {
            imageView.layer?.addSublayer(maskLayer)
        }
        // ROI layer covers full imageView (path coords are absolute within it)
        roiLayer.frame = imageView.bounds
        // Redraw ROI path for new bounds
        if let roi = currentRoi {
            drawRoiOverlay(roi)
        }
        // Mask layer must track letterbox rect
        if maskLayer.contents != nil {
            maskLayer.frame = imageRectInView()
        }
    }

    // MARK: - File loading

    @objc private func handleFileSelected(_ notification: Notification) {
        guard let url = notification.userInfo?["url"] as? URL else { return }

        sessionId += 1
        let session = sessionId
        statusLabel.stringValue = "Loading \(url.lastPathComponent)..."
        clearOverlays()

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            do {
                let loadedFrames = try loadFile(path: url.path)
                DispatchQueue.main.async {
                    guard self?.sessionId == session else { return }
                    self?.displayFrames(loadedFrames, filename: url.lastPathComponent)
                }
            } catch {
                DispatchQueue.main.async {
                    guard self?.sessionId == session else { return }
                    self?.statusLabel.stringValue = "Error: \(error.localizedDescription)"
                }
            }
        }
    }

    private func displayFrames(_ loadedFrames: [FfiFrame], filename: String) {
        frames = loadedFrames
        currentIndex = 0

        guard let first = frames.first else {
            statusLabel.stringValue = "No frames in \(filename)"
            return
        }

        let colorspace: String
        switch first.channels {
        case 1: colorspace = "Grayscale"
        case 3: colorspace = "RGB"
        case 4: colorspace = "RGBA"
        default: colorspace = "?\(first.channels)ch"
        }

        statusLabel.stringValue = "\(filename) | \(first.width)x\(first.height) | \(colorspace) | \(frames.count) frame(s)"

        if frames.count > 1 {
            sequenceSlider.isHidden = false
            frameLabel.isHidden = false
            sequenceSlider.maxValue = Double(frames.count - 1)
            sequenceSlider.integerValue = 0
            sequenceSlider.numberOfTickMarks = min(frames.count, 100)
            updateFrameLabel()
        } else {
            sequenceSlider.isHidden = true
            frameLabel.isHidden = true
        }

        showFrame(at: 0)

        NotificationCenter.default.post(
            name: .muddFramesLoaded, object: nil,
            userInfo: ["frames": frames]
        )
    }

    // MARK: - Frame display

    @objc private func sliderChanged() {
        let idx = sequenceSlider.integerValue
        guard idx != currentIndex, idx >= 0, idx < frames.count else { return }
        currentIndex = idx
        showFrame(at: idx)
        updateFrameLabel()
        clearOverlays()
        NotificationCenter.default.post(
            name: .muddCurrentIndexChanged, object: nil,
            userInfo: ["index": idx]
        )
    }

    private func updateFrameLabel() {
        frameLabel.stringValue = "\(currentIndex + 1) / \(frames.count)"
    }

    private func showFrame(at index: Int) {
        guard index < frames.count else { return }
        let frame = frames[index]
        currentIndex = index

        guard let nsImage = makeNSImage(from: frame) else {
            statusLabel.stringValue = "Failed to create image from frame data"
            return
        }
        imageView.image = nsImage
    }

    // MARK: - ROI overlay

    @objc private func handleRoiDetected(_ notification: Notification) {
        if notification.userInfo?["roi"] is NSNull {
            currentRoi = nil
            clearOverlays()
            return
        }
        guard let roi = notification.userInfo?["roi"] as? FfiRoi else { return }
        currentRoi = roi
        drawRoiOverlay(roi)
    }

    private func drawRoiOverlay(_ roi: FfiRoi) {
        guard !frames.isEmpty else { return }
        let frame = frames[currentIndex]

        let imageRect = imageRectInView()
        let scaleX = imageRect.width / CGFloat(frame.width)
        let scaleY = imageRect.height / CGFloat(frame.height)

        let roiRect = CGRect(
            x: imageRect.origin.x + CGFloat(roi.x) * scaleX,
            y: imageRect.origin.y + CGFloat(roi.y) * scaleY,
            width: CGFloat(roi.width) * scaleX,
            height: CGFloat(roi.height) * scaleY
        )

        roiLayer.path = CGPath(rect: roiRect, transform: nil)
    }

    @objc private func handleFrameUpdated(_ notification: Notification) {
        guard let frame = notification.userInfo?["frame"] as? FfiFrame,
              let index = notification.userInfo?["index"] as? Int else { return }

        if index < frames.count {
            frames[index] = frame
        }
        clearOverlays()
        showFrame(at: index)

        // Update status
        let colorspace: String
        switch frame.channels {
        case 1: colorspace = "Grayscale"
        case 3: colorspace = "RGB"
        case 4: colorspace = "RGBA"
        default: colorspace = "?\(frame.channels)ch"
        }
        statusLabel.stringValue = "Cropped | \(frame.width)x\(frame.height) | \(colorspace)"
    }

    private func clearOverlays() {
        currentRoi = nil
        roiLayer.path = nil
        maskLayer.contents = nil
    }

    // MARK: - Drag-to-select ROI

    override func mouseDown(with event: NSEvent) {
        let point = imageView.convert(event.locationInWindow, from: nil)
        guard imageView.bounds.contains(point) else {
            super.mouseDown(with: event)
            return
        }

        if segModeActive {
            handleSegClick(at: point)
            return
        }

        dragStart = point
        isDragging = true
    }

    override func mouseDragged(with event: NSEvent) {
        guard isDragging, let start = dragStart else { return }
        let current = imageView.convert(event.locationInWindow, from: nil)

        let rect = CGRect(
            x: min(start.x, current.x),
            y: min(start.y, current.y),
            width: abs(current.x - start.x),
            height: abs(current.y - start.y)
        )

        roiLayer.path = CGPath(rect: rect, transform: nil)
    }

    override func mouseUp(with event: NSEvent) {
        guard isDragging, let start = dragStart else { return }
        isDragging = false
        let end = imageView.convert(event.locationInWindow, from: nil)

        let viewRect = CGRect(
            x: min(start.x, end.x),
            y: min(start.y, end.y),
            width: abs(end.x - start.x),
            height: abs(end.y - start.y)
        )

        // Ignore tiny drags (accidental clicks)
        guard viewRect.width > 5, viewRect.height > 5 else {
            dragStart = nil
            return
        }

        // Convert view coordinates to image pixel coordinates
        guard let roi = viewRectToRoi(viewRect) else {
            dragStart = nil
            return
        }

        NotificationCenter.default.post(
            name: .muddRoiManual, object: nil,
            userInfo: ["roi": roi]
        )
        dragStart = nil
    }

    private func viewRectToRoi(_ viewRect: CGRect) -> FfiRoi? {
        guard !frames.isEmpty else { return nil }
        let frame = frames[currentIndex]
        let imageRect = imageRectInView()
        guard imageRect.width > 0, imageRect.height > 0 else { return nil }

        let scaleX = CGFloat(frame.width) / imageRect.width
        let scaleY = CGFloat(frame.height) / imageRect.height

        let x = max(0, (viewRect.origin.x - imageRect.origin.x) * scaleX)
        let y = max(0, (viewRect.origin.y - imageRect.origin.y) * scaleY)
        let w = min(CGFloat(frame.width) - x, viewRect.width * scaleX)
        let h = min(CGFloat(frame.height) - y, viewRect.height * scaleY)

        guard w > 0, h > 0 else { return nil }

        return FfiRoi(x: UInt32(x), y: UInt32(y), width: UInt32(w), height: UInt32(h))
    }

    // MARK: - Segmentation

    @objc private func handleSegModeChanged(_ notification: Notification) {
        guard let active = notification.userInfo?["active"] as? Bool else { return }
        segModeActive = active
        if active {
            NSCursor.crosshair.push()
        } else {
            NSCursor.pop()
        }
    }

    private func handleSegClick(at viewPoint: NSPoint) {
        guard !frames.isEmpty, isEngineReady() else { return }
        let idx = currentIndex
        let frame = frames[idx]
        let imageRect = imageRectInView()
        guard imageRect.width > 0, imageRect.height > 0 else { return }

        let scaleX = CGFloat(frame.width) / imageRect.width
        let scaleY = CGFloat(frame.height) / imageRect.height

        let imgX = Float((viewPoint.x - imageRect.origin.x) * scaleX)
        let imgY = Float((viewPoint.y - imageRect.origin.y) * scaleY)

        let prompt = FfiPromptPoint(x: imgX, y: imgY, label: 1)
        let session = sessionId
        statusLabel.stringValue = "Segmenting..."

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            do {
                let masks = try segmentFrame(frame: frame, prompts: [prompt])
                DispatchQueue.main.async {
                    guard self?.sessionId == session else { return }
                    self?.displayMasks(masks)
                    self?.statusLabel.stringValue = "Segmentation: \(masks.count) mask(s)"
                    NotificationCenter.default.post(
                        name: .muddMasksUpdated, object: nil,
                        userInfo: ["masks": masks, "index": idx]
                    )
                }
            } catch {
                DispatchQueue.main.async {
                    guard self?.sessionId == session else { return }
                    self?.statusLabel.stringValue = "Segmentation failed: \(error.localizedDescription)"
                }
            }
        }
    }

    private func displayMasks(_ masks: [FfiMask]) {
        guard let first = masks.first else { return }

        let w = Int(first.width)
        let h = Int(first.height)
        let pixelCount = w * h

        // Guard: mask data must have at least w*h bytes
        guard first.data.count >= pixelCount else {
            statusLabel.stringValue = "Mask data too short (\(first.data.count) < \(pixelCount))"
            return
        }

        // Create mask image (green overlay)
        var rgba = Data(count: pixelCount * 4)

        for i in 0 ..< pixelCount {
            let val = first.data[i]
            if val > 127 {
                rgba[i * 4] = 0       // R
                rgba[i * 4 + 1] = 200 // G
                rgba[i * 4 + 2] = 0   // B
                rgba[i * 4 + 3] = 140 // A
            }
        }

        guard let provider = CGDataProvider(data: rgba as CFData),
              let cgImage = CGImage(
                  width: w, height: h,
                  bitsPerComponent: 8,
                  bitsPerPixel: 32,
                  bytesPerRow: w * 4,
                  space: CGColorSpaceCreateDeviceRGB(),
                  bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.last.rawValue),
                  provider: provider,
                  decode: nil,
                  shouldInterpolate: false,
                  intent: .defaultIntent
              ) else { return }

        // Position mask layer to match image rect (letterbox-aware)
        let imgRect = imageRectInView()
        maskLayer.contents = cgImage
        maskLayer.frame = imgRect
    }

    // MARK: - Helpers

    private func imageRectInView() -> CGRect {
        guard let image = imageView.image else { return .zero }
        let imageSize = image.size
        let viewSize = imageView.bounds.size
        guard viewSize.width > 0, viewSize.height > 0 else { return .zero }

        let imageAspect = imageSize.width / imageSize.height
        let viewAspect = viewSize.width / viewSize.height

        var drawSize: CGSize
        if imageAspect > viewAspect {
            drawSize = CGSize(width: viewSize.width, height: viewSize.width / imageAspect)
        } else {
            drawSize = CGSize(width: viewSize.height * imageAspect, height: viewSize.height)
        }

        let x = (viewSize.width - drawSize.width) / 2
        let y = (viewSize.height - drawSize.height) / 2

        return CGRect(origin: CGPoint(x: x, y: y), size: drawSize)
    }

    private func makeNSImage(from frame: FfiFrame) -> NSImage? {
        let w = Int(frame.width)
        let h = Int(frame.height)
        let ch = Int(frame.channels)

        let bitsPerComponent = 8
        let bitsPerPixel = bitsPerComponent * ch
        let bytesPerRow = w * ch

        let colorSpace: CGColorSpace
        let bitmapInfo: CGBitmapInfo

        switch ch {
        case 1:
            colorSpace = CGColorSpaceCreateDeviceGray()
            bitmapInfo = CGBitmapInfo(rawValue: 0)
        case 3:
            colorSpace = CGColorSpaceCreateDeviceRGB()
            bitmapInfo = CGBitmapInfo(rawValue: 0)
        case 4:
            colorSpace = CGColorSpaceCreateDeviceRGB()
            bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.last.rawValue)
        default:
            return nil
        }

        guard frame.data.count >= w * h * ch else { return nil }

        guard let provider = CGDataProvider(data: frame.data as CFData) else {
            return nil
        }
        guard let cgImage = CGImage(
            width: w, height: h,
            bitsPerComponent: bitsPerComponent,
            bitsPerPixel: bitsPerPixel,
            bytesPerRow: bytesPerRow,
            space: colorSpace,
            bitmapInfo: bitmapInfo,
            provider: provider,
            decode: nil,
            shouldInterpolate: true,
            intent: .defaultIntent
        ) else { return nil }

        return NSImage(cgImage: cgImage, size: NSSize(width: w, height: h))
    }
}
