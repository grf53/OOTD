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

private typealias FreeStringFn = @convention(c) (UnsafeMutablePointer<CChar>?) -> Void

private final class NativeFFI {
    let handle: UnsafeMutableRawPointer
    let between: BetweenFn
    let fromDuration: FromDurationFn
    let freeString: FreeStringFn

    init() throws {
        let paths = Self.candidateLibraryPaths()

        var lastOpenError: OOTDError?
        for path in paths where FileManager.default.fileExists(atPath: path) {
            let flags = Int32(RTLD_NOW | RTLD_LOCAL)
            guard let handle = dlopen(path, flags) else {
                lastOpenError = .failedToOpenLibrary(path: path, reason: Self.currentDlError())
                continue
            }

            do {
                self.handle = handle
                self.between = try Self.loadSymbol(handle: handle, name: "ootd_between_rfc3339_with_options")
                self.fromDuration = try Self.loadSymbol(handle: handle, name: "ootd_from_duration_parts_with_options")
                self.freeString = try Self.loadSymbol(handle: handle, name: "ootd_free_string")
                return
            } catch {
                dlclose(handle)
                throw error
            }
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
}
