import Foundation

#if canImport(Darwin)
import Darwin
#elseif canImport(Glibc)
import Glibc
#endif

public enum OOTDLocale: String {
    case en
    case ko
}

public enum OOTDError: Error, CustomStringConvertible {
    case invalidDuration(String)
    case libraryNotFound([String])
    case failedToOpenLibrary(path: String, reason: String)
    case symbolNotFound(String)
    case nativeCallFailed(String)

    public var description: String {
        switch self {
        case let .invalidDuration(message):
            return message
        case let .libraryNotFound(paths):
            return "OOTD native library not found. Tried: \(paths.joined(separator: ", "))"
        case let .failedToOpenLibrary(path, reason):
            return "Failed to open OOTD native library at \(path): \(reason)"
        case let .symbolNotFound(name):
            return "Missing native symbol: \(name)"
        case let .nativeCallFailed(message):
            return message
        }
    }
}

public struct OOTDDurationRange: Equatable {
    public let start: Duration
    public let end: Duration

    public init(start: Duration, end: Duration) {
        self.start = start
        self.end = end
    }

    public func resolveAt(_ anchorRFC3339: String) throws -> OOTDTimestampRange {
        try OOTD.resolveDurationRangeAt(self, anchorRFC3339: anchorRFC3339)
    }
}

public struct OOTDTimestampRange: Equatable {
    public let start: Date
    public let end: Date

    public init(
        start: Date,
        end: Date
    ) {
        self.start = start
        self.end = end
    }
}

public struct OOTDExpressionCandidate: Equatable {
    public let start: Int
    public let end: Int
    public let text: String
}

private typealias BetweenFn = @convention(c) (
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
    Bool
) -> UnsafeMutablePointer<CChar>?

private typealias FromDurationFn = @convention(c) (
    Int64,
    Bool,
    UnsafePointer<CChar>?,
    Bool
) -> UnsafeMutablePointer<CChar>?

private typealias RangeOfFn = @convention(c) (
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<Int64>?,
    UnsafeMutablePointer<Int64>?
) -> Bool

private typealias ResolveDurationRangeAtRfc3339Fn = @convention(c) (
    Int64,
    Int64,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool

private typealias FreeStringFn = @convention(c) (UnsafeMutablePointer<CChar>?) -> Void

private final class NativeFFI {
    let handle: UnsafeMutableRawPointer
    let between: BetweenFn
    let fromDuration: FromDurationFn
    let rangeOf: RangeOfFn
    let resolveDurationRangeAtRfc3339: ResolveDurationRangeAtRfc3339Fn
    let freeString: FreeStringFn

    init() throws {
        let paths = Self.candidateLibraryPaths()
        typealias LoadedSymbols = (
            handle: UnsafeMutableRawPointer,
            between: BetweenFn,
            fromDuration: FromDurationFn,
            rangeOf: RangeOfFn,
            resolveDurationRangeAtRfc3339: ResolveDurationRangeAtRfc3339Fn,
            freeString: FreeStringFn
        )
        var loaded: LoadedSymbols?

        var lastOpenError: OOTDError?
        for path in paths where FileManager.default.fileExists(atPath: path) {
            let flags = Int32(RTLD_NOW | RTLD_LOCAL)
            guard let handle = dlopen(path, flags) else {
                lastOpenError = .failedToOpenLibrary(path: path, reason: Self.currentDlError())
                continue
            }

            do {
                let between: BetweenFn = try Self.loadSymbol(
                    handle: handle,
                    name: "ootd_between_rfc3339_with_options"
                )
                let fromDuration: FromDurationFn = try Self.loadSymbol(
                    handle: handle,
                    name: "ootd_from_duration_parts_with_options"
                )
                let rangeOf: RangeOfFn = try Self.loadSymbol(
                    handle: handle,
                    name: "ootd_range_of"
                )
                let resolveDurationRangeAtRfc3339: ResolveDurationRangeAtRfc3339Fn = try Self.loadSymbol(
                    handle: handle,
                    name: "ootd_duration_range_resolve_at_rfc3339"
                )
                let freeString: FreeStringFn = try Self.loadSymbol(handle: handle, name: "ootd_free_string")

                loaded = (
                    handle: handle,
                    between: between,
                    fromDuration: fromDuration,
                    rangeOf: rangeOf,
                    resolveDurationRangeAtRfc3339: resolveDurationRangeAtRfc3339,
                    freeString: freeString
                )
                break
            } catch {
                if let ootdError = error as? OOTDError {
                    lastOpenError = ootdError
                } else {
                    lastOpenError = .nativeCallFailed(String(describing: error))
                }
                dlclose(handle)
                continue
            }
        }

        if let loaded {
            self.handle = loaded.handle
            self.between = loaded.between
            self.fromDuration = loaded.fromDuration
            self.rangeOf = loaded.rangeOf
            self.resolveDurationRangeAtRfc3339 = loaded.resolveDurationRangeAtRfc3339
            self.freeString = loaded.freeString
            return
        }

        if let lastOpenError {
            throw lastOpenError
        }

        throw OOTDError.libraryNotFound(paths)
    }

    deinit {
        dlclose(handle)
    }

    private static func loadSymbol<T>(handle: UnsafeMutableRawPointer, name: String) throws -> T {
        dlerror()
        guard let symbol = dlsym(handle, name) else {
            throw OOTDError.symbolNotFound(name)
        }
        return unsafeBitCast(symbol, to: T.self)
    }

    private static func currentDlError() -> String {
        guard let err = dlerror() else {
            return "unknown error"
        }
        return String(cString: err)
    }

    private static func candidateLibraryPaths() -> [String] {
        if let configured = ProcessInfo.processInfo.environment["OOTD_FFI_LIB_PATH"], !configured.isEmpty {
            return [configured]
        }

        let libName = nativeLibraryFileName()
        let cwd = FileManager.default.currentDirectoryPath
        return [
            "\(cwd)/target/debug/\(libName)",
            "\(cwd)/../target/debug/\(libName)",
            "\(cwd)/../../target/debug/\(libName)",
            "\(cwd)/../../../target/debug/\(libName)",
        ]
    }

    private static func nativeLibraryFileName() -> String {
        #if os(macOS)
        return "libootd_ffi_c.dylib"
        #elseif os(Windows)
        return "ootd_ffi_c.dll"
        #else
        return "libootd_ffi_c.so"
        #endif
    }
}

public enum OOTD {
    private static let ffiResult: Result<NativeFFI, Error> = Result { try NativeFFI() }

    private static func ffi() throws -> NativeFFI {
        try ffiResult.get()
    }

    public static func between(
        startRFC3339: String,
        endRFC3339: String,
        locale: OOTDLocale = .en,
        useNativeKoNumber: Bool = false
    ) throws -> String {
        let ffi = try ffi()

        return try startRFC3339.withCString { startPtr in
            try endRFC3339.withCString { endPtr in
                try locale.rawValue.withCString { localePtr in
                    guard let raw = ffi.between(startPtr, endPtr, localePtr, useNativeKoNumber) else {
                        throw OOTDError.nativeCallFailed("Native between call returned null")
                    }

                    defer { ffi.freeString(raw) }
                    return String(cString: raw)
                }
            }
        }
    }

    public static func between(
        start: Date,
        end: Date,
        locale: OOTDLocale = .en,
        timeZone: TimeZone = TimeZone(secondsFromGMT: 0)!,
        useNativeKoNumber: Bool = false
    ) throws -> String {
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime]
        formatter.timeZone = timeZone

        let startRFC3339 = formatter.string(from: start)
        let endRFC3339 = formatter.string(from: end)
        return try between(
            startRFC3339: startRFC3339,
            endRFC3339: endRFC3339,
            locale: locale,
            useNativeKoNumber: useNativeKoNumber
        )
    }

    public static func fromDuration(
        seconds: Int64,
        isFuture: Bool = false,
        locale: OOTDLocale = .en,
        useNativeKoNumber: Bool = false
    ) throws -> String {
        if seconds < 0 {
            throw OOTDError.invalidDuration("negative duration is not allowed: \(seconds)")
        }

        let ffi = try ffi()

        return try locale.rawValue.withCString { localePtr in
            guard let raw = ffi.fromDuration(seconds, isFuture, localePtr, useNativeKoNumber) else {
                throw OOTDError.nativeCallFailed("Native fromDuration call returned null")
            }

            defer { ffi.freeString(raw) }
            return String(cString: raw)
        }
    }

    public static func fromDuration(
        timeIntervalSeconds: Double,
        isFuture: Bool = false,
        locale: OOTDLocale = .en,
        useNativeKoNumber: Bool = false
    ) throws -> String {
        guard timeIntervalSeconds.isFinite else {
            throw OOTDError.invalidDuration("Duration must be finite")
        }
        if timeIntervalSeconds < Double(Int64.min) || timeIntervalSeconds > Double(Int64.max) {
            throw OOTDError.invalidDuration("Duration is out of Int64 range")
        }

        let seconds = Int64(timeIntervalSeconds.rounded(.towardZero))
        return try fromDuration(
            seconds: seconds,
            isFuture: isFuture,
            locale: locale,
            useNativeKoNumber: useNativeKoNumber
        )
    }

    public static func rangeOf(
        expression: String,
        locale: OOTDLocale = .en
    ) throws -> OOTDDurationRange {
        let ffi = try ffi()

        var startSeconds: Int64 = 0
        var endSeconds: Int64 = 0

        let ok = expression.withCString { expressionPtr in
            locale.rawValue.withCString { localePtr in
                ffi.rangeOf(expressionPtr, localePtr, &startSeconds, &endSeconds)
            }
        }

        guard ok else {
            throw OOTDError.nativeCallFailed("Native rangeOf call failed")
        }

        return OOTDDurationRange(
            start: .seconds(Double(startSeconds)),
            end: .seconds(Double(endSeconds))
        )
    }

    public static func extractExpressions(
        input: String,
        locale: OOTDLocale = .en
    ) -> [OOTDExpressionCandidate] {
        let patterns: [String]
        switch locale {
        case .ko:
            patterns = [
                #"([0-9]{1,3}|[가-힣]{1,8})\s*(년|달|주|일|시간|분|초)(\s*반)?\s*(전|후)"#,
                #"(어제|오늘|내일)\s*(새벽|아침|낮|저녁|밤)"#,
            ]
        case .en:
            patterns = [
                #"(?i)\b(a|an|\d+)\s+(year|years|month|months|week|weeks|day|days|hour|hours|minute|minutes|second|seconds)(\s+and\s+a\s+half)?\s+(ago|later)\b"#,
                #"(?i)\b(yesterday|this|tomorrow)\s+(dawn|morning|afternoon|evening|night)\b"#,
                #"(?i)\b(last night|earlier tonight|tonight)\b"#,
            ]
        }

        var raw: [OOTDExpressionCandidate] = []
        for pattern in patterns {
            guard let re = try? NSRegularExpression(pattern: pattern) else { continue }
            for m in re.matches(in: input, range: NSRange(input.startIndex..., in: input)) {
                guard let range = Range(m.range, in: input) else { continue }
                let text = String(input[range]).trimmingCharacters(in: .whitespacesAndNewlines)
                if text.isEmpty {
                    continue
                }
                if (try? rangeOf(expression: text, locale: locale)) != nil {
                    raw.append(
                        OOTDExpressionCandidate(
                            start: m.range.location,
                            end: m.range.location + m.range.length,
                            text: text
                        )
                    )
                }
            }
        }

        raw.sort {
            let len0 = $0.end - $0.start
            let len1 = $1.end - $1.start
            if len0 != len1 {
                return len0 > len1
            }
            return $0.start < $1.start
        }

        var selected: [OOTDExpressionCandidate] = []
        for c in raw {
            let overlap = selected.contains(where: { s in s.start < c.end && c.start < s.end })
            if !overlap {
                selected.append(c)
            }
        }
        selected.sort { $0.start < $1.start }
        return selected
    }

    fileprivate static func resolveDurationRangeAt(
        _ range: OOTDDurationRange,
        anchorRFC3339: String
    ) throws -> OOTDTimestampRange {
        let ffi = try ffi()
        var startRaw: UnsafeMutablePointer<CChar>? = nil
        var endRaw: UnsafeMutablePointer<CChar>? = nil

        let ok = anchorRFC3339.withCString { anchorPtr in
            ffi.resolveDurationRangeAtRfc3339(
                range.start.components.seconds,
                range.end.components.seconds,
                anchorPtr,
                &startRaw,
                &endRaw
            )
        }

        guard ok else {
            throw OOTDError.nativeCallFailed("Native duration range resolve call failed")
        }

        guard let startRaw else {
            throw OOTDError.nativeCallFailed("Native duration range resolve returned null start")
        }
        guard let endRaw else {
            ffi.freeString(startRaw)
            throw OOTDError.nativeCallFailed("Native duration range resolve returned null end")
        }

        defer {
            ffi.freeString(startRaw)
            ffi.freeString(endRaw)
        }

        let startText = String(cString: startRaw)
        let endText = String(cString: endRaw)
        guard let start = parseRFC3339(startText) else {
            throw OOTDError.nativeCallFailed("Native duration range resolve returned invalid start RFC3339")
        }
        guard let end = parseRFC3339(endText) else {
            throw OOTDError.nativeCallFailed("Native duration range resolve returned invalid end RFC3339")
        }

        return OOTDTimestampRange(start: start, end: end)
    }

    private static func parseRFC3339(_ value: String) -> Date? {
        let basic = ISO8601DateFormatter()
        basic.formatOptions = [.withInternetDateTime]
        if let out = basic.date(from: value) {
            return out
        }

        let fractional = ISO8601DateFormatter()
        fractional.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        return fractional.date(from: value)
    }
}
