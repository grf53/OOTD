package io.ootd;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.invoke.MethodHandle;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.time.Duration;
import java.time.OffsetDateTime;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Objects;

import static java.lang.foreign.ValueLayout.ADDRESS;
import static java.lang.foreign.ValueLayout.JAVA_BOOLEAN;
import static java.lang.foreign.ValueLayout.JAVA_LONG;

public final class Ootd {
    private static final Linker LINKER = Linker.nativeLinker();
    private static final Arena LOOKUP_ARENA = Arena.ofShared();
    private static final SymbolLookup LOOKUP = initLookup();

    private static final MethodHandle BETWEEN_WITH_OPTIONS = downcall(
            "ootd_between_rfc3339_with_options",
            FunctionDescriptor.of(ADDRESS, ADDRESS, ADDRESS, ADDRESS, JAVA_BOOLEAN)
    );

    private static final MethodHandle FROM_DURATION_WITH_OPTIONS = downcall(
            "ootd_from_duration_parts_with_options",
            FunctionDescriptor.of(ADDRESS, JAVA_LONG, JAVA_BOOLEAN, ADDRESS, JAVA_BOOLEAN)
    );

    private static final MethodHandle FREE = downcall(
            "ootd_free_string",
            FunctionDescriptor.ofVoid(ADDRESS)
    );

    private Ootd() {}

    private static SymbolLookup initLookup() {
        String configuredPath = System.getProperty("ootd.ffi.lib.path");
        if (configuredPath == null || configuredPath.isBlank()) {
            configuredPath = System.getenv("OOTD_FFI_LIB_PATH");
        }

        List<Path> candidates = new ArrayList<>();
        if (configuredPath != null && !configuredPath.isBlank()) {
            candidates.add(Path.of(configuredPath));
        } else {
            String libName = nativeLibraryFileName();
            candidates.add(Path.of("target", "debug", libName));
            candidates.add(Path.of("..", "target", "debug", libName));
            candidates.add(Path.of("..", "..", "target", "debug", libName));
            candidates.add(Path.of("..", "..", "..", "target", "debug", libName));
        }

        Throwable lastError = null;
        for (Path candidate : candidates) {
            Path absolute = candidate.toAbsolutePath().normalize();
            if (!Files.exists(absolute)) {
                continue;
            }

            try {
                return SymbolLookup.libraryLookup(absolute, LOOKUP_ARENA);
            } catch (Throwable t) {
                lastError = t;
            }
        }

        StringBuilder searched = new StringBuilder();
        for (Path candidate : candidates) {
            if (searched.length() > 0) {
                searched.append(", ");
            }
            searched.append(candidate.toAbsolutePath().normalize());
        }

        IllegalStateException ex = new IllegalStateException(
                "Failed to initialize OOTD native lookup. Searched: " + searched
        );
        if (lastError != null) {
            ex.initCause(lastError);
        }
        throw ex;
    }

    private static String nativeLibraryFileName() {
        String os = System.getProperty("os.name", "").toLowerCase();
        if (os.contains("mac")) {
            return "libootd_ffi_c.dylib";
        }
        if (os.contains("win")) {
            return "ootd_ffi_c.dll";
        }
        return "libootd_ffi_c.so";
    }

    public static String between(OffsetDateTime start, OffsetDateTime end, OotdLocale locale) {
        return between(start, end, locale, false);
    }

    public static String between(
            OffsetDateTime start,
            OffsetDateTime end,
            OotdLocale locale,
            boolean useNativeKoNumber
    ) {
        Objects.requireNonNull(start, "start must not be null");
        Objects.requireNonNull(end, "end must not be null");
        OotdLocale safeLocale = locale == null ? OotdLocale.EN : locale;
        return between(
                toRfc3339(start),
                toRfc3339(end),
                safeLocale.code(),
                useNativeKoNumber
        );
    }

    public static String between(ZonedDateTime start, ZonedDateTime end, OotdLocale locale) {
        return between(start, end, locale, false);
    }

    public static String between(
            ZonedDateTime start,
            ZonedDateTime end,
            OotdLocale locale,
            boolean useNativeKoNumber
    ) {
        Objects.requireNonNull(start, "start must not be null");
        Objects.requireNonNull(end, "end must not be null");
        OotdLocale safeLocale = locale == null ? OotdLocale.EN : locale;
        return between(
                toRfc3339(start),
                toRfc3339(end),
                safeLocale.code(),
                useNativeKoNumber
        );
    }

    public static String between(String startRfc3339, String endRfc3339, OotdLocale locale) {
        return between(startRfc3339, endRfc3339, locale, false);
    }

    public static String between(
            String startRfc3339,
            String endRfc3339,
            OotdLocale locale,
            boolean useNativeKoNumber
    ) {
        OotdLocale safeLocale = locale == null ? OotdLocale.EN : locale;
        return between(startRfc3339, endRfc3339, safeLocale.code(), useNativeKoNumber);
    }

    public static String between(String startRfc3339, String endRfc3339, String locale) {
        return between(startRfc3339, endRfc3339, locale, false);
    }

    public static String between(String startRfc3339, String endRfc3339, String locale, boolean useNativeKoNumber) {
        Objects.requireNonNull(startRfc3339, "startRfc3339 must not be null");
        Objects.requireNonNull(endRfc3339, "endRfc3339 must not be null");
        String safeLocale = locale == null ? "en" : locale;

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment start = arena.allocateFrom(startRfc3339);
            MemorySegment end = arena.allocateFrom(endRfc3339);
            MemorySegment localePtr = arena.allocateFrom(safeLocale);

            MemorySegment raw = (MemorySegment) BETWEEN_WITH_OPTIONS.invoke(
                    start,
                    end,
                    localePtr,
                    useNativeKoNumber
            );
            return consumeNativeString(raw);
        } catch (Throwable t) {
            throw new IllegalArgumentException("Failed to render OOTD string", t);
        }
    }

    public static String fromDuration(long seconds, boolean isFuture, String locale) {
        return fromDuration(seconds, isFuture, locale, false);
    }

    public static String fromDuration(Duration duration, boolean isFuture, OotdLocale locale) {
        return fromDuration(duration, isFuture, locale, false);
    }

    public static String fromDuration(
            Duration duration,
            boolean isFuture,
            OotdLocale locale,
            boolean useNativeKoNumber
    ) {
        Objects.requireNonNull(duration, "duration must not be null");
        OotdLocale safeLocale = locale == null ? OotdLocale.EN : locale;
        return fromDuration(duration.getSeconds(), isFuture, safeLocale.code(), useNativeKoNumber);
    }

    public static String fromDuration(long seconds, boolean isFuture, OotdLocale locale) {
        return fromDuration(seconds, isFuture, locale, false);
    }

    public static String fromDuration(
            long seconds,
            boolean isFuture,
            OotdLocale locale,
            boolean useNativeKoNumber
    ) {
        OotdLocale safeLocale = locale == null ? OotdLocale.EN : locale;
        return fromDuration(seconds, isFuture, safeLocale.code(), useNativeKoNumber);
    }

    public static String fromDuration(long seconds, boolean isFuture, String locale, boolean useNativeKoNumber) {
        if (seconds < 0) {
            throw new IllegalArgumentException("negative duration is not allowed: " + seconds);
        }
        String safeLocale = locale == null ? "en" : locale;

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment localePtr = arena.allocateFrom(safeLocale);
            MemorySegment raw = (MemorySegment) FROM_DURATION_WITH_OPTIONS.invoke(
                    seconds,
                    isFuture,
                    localePtr,
                    useNativeKoNumber
            );
            return consumeNativeString(raw);
        } catch (Throwable t) {
            throw new IllegalArgumentException("Failed to render OOTD duration", t);
        }
    }

    private static MethodHandle downcall(String symbol, FunctionDescriptor descriptor) {
        MemorySegment addr = LOOKUP.find(symbol)
                .orElseThrow(() -> new UnsatisfiedLinkError("Missing native symbol: " + symbol));
        return LINKER.downcallHandle(addr, descriptor);
    }

    private static String toRfc3339(OffsetDateTime value) {
        return value.format(DateTimeFormatter.ISO_OFFSET_DATE_TIME);
    }

    private static String toRfc3339(ZonedDateTime value) {
        return value.toOffsetDateTime().format(DateTimeFormatter.ISO_OFFSET_DATE_TIME);
    }

    private static String consumeNativeString(MemorySegment raw) throws Throwable {
        if (raw.address() == 0) {
            throw new IllegalArgumentException("Native OOTD call returned null");
        }

        MemorySegment cstr = raw.reinterpret(Long.MAX_VALUE);
        String out = cstr.getString(0);
        FREE.invoke(raw);
        return out;
    }
}
