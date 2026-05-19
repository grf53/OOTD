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
import java.util.regex.Matcher;
import java.util.regex.Pattern;

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

    private static final MethodHandle RANGE_OF = downcall(
            "ootd_range_of",
            FunctionDescriptor.of(JAVA_BOOLEAN, ADDRESS, ADDRESS, ADDRESS, ADDRESS)
    );

    private static final MethodHandle RESOLVE_DURATION_RANGE_AT_RFC3339 = downcall(
            "ootd_duration_range_resolve_at_rfc3339",
            FunctionDescriptor.of(JAVA_BOOLEAN, JAVA_LONG, JAVA_LONG, ADDRESS, ADDRESS, ADDRESS)
    );

    private static final MethodHandle FREE = downcall(
            "ootd_free_string",
            FunctionDescriptor.ofVoid(ADDRESS)
    );

    public record DurationRange(Duration start, Duration end) {
        public TimestampRange resolveAt(String anchorRfc3339) {
            return Ootd.resolveDurationRangeAtRfc3339(this, anchorRfc3339);
        }

        public TimestampRange resolveAt(OffsetDateTime anchor) {
            Objects.requireNonNull(anchor, "anchor must not be null");
            return resolveAt(toRfc3339(anchor));
        }

        public TimestampRange resolveAt(ZonedDateTime anchor) {
            Objects.requireNonNull(anchor, "anchor must not be null");
            return resolveAt(toRfc3339(anchor));
        }
    }

    public record TimestampRange(
            OffsetDateTime start,
            OffsetDateTime end
    ) {}

    public record ExpressionCandidate(
            int start,
            int end,
            String text
    ) {}

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

    public static String between(OffsetDateTime start, OffsetDateTime end, Locale locale) {
        return between(start, end, locale, false);
    }

    public static String between(
            OffsetDateTime start,
            OffsetDateTime end,
            Locale locale,
            boolean useNativeKoNumber
    ) {
        Objects.requireNonNull(start, "start must not be null");
        Objects.requireNonNull(end, "end must not be null");
        Locale safeLocale = locale == null ? Locale.EN : locale;
        return between(
                toRfc3339(start),
                toRfc3339(end),
                safeLocale.code(),
                useNativeKoNumber
        );
    }

    public static String between(ZonedDateTime start, ZonedDateTime end, Locale locale) {
        return between(start, end, locale, false);
    }

    public static String between(
            ZonedDateTime start,
            ZonedDateTime end,
            Locale locale,
            boolean useNativeKoNumber
    ) {
        Objects.requireNonNull(start, "start must not be null");
        Objects.requireNonNull(end, "end must not be null");
        Locale safeLocale = locale == null ? Locale.EN : locale;
        return between(
                toRfc3339(start),
                toRfc3339(end),
                safeLocale.code(),
                useNativeKoNumber
        );
    }

    public static String between(String startRfc3339, String endRfc3339, Locale locale) {
        return between(startRfc3339, endRfc3339, locale, false);
    }

    public static String between(
            String startRfc3339,
            String endRfc3339,
            Locale locale,
            boolean useNativeKoNumber
    ) {
        Locale safeLocale = locale == null ? Locale.EN : locale;
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

    public static String fromDuration(Duration duration, boolean isFuture, Locale locale) {
        return fromDuration(duration, isFuture, locale, false);
    }

    public static String fromDuration(
            Duration duration,
            boolean isFuture,
            Locale locale,
            boolean useNativeKoNumber
    ) {
        Objects.requireNonNull(duration, "duration must not be null");
        Locale safeLocale = locale == null ? Locale.EN : locale;
        return fromDuration(duration.getSeconds(), isFuture, safeLocale.code(), useNativeKoNumber);
    }

    public static String fromDuration(long seconds, boolean isFuture, Locale locale) {
        return fromDuration(seconds, isFuture, locale, false);
    }

    public static String fromDuration(
            long seconds,
            boolean isFuture,
            Locale locale,
            boolean useNativeKoNumber
    ) {
        Locale safeLocale = locale == null ? Locale.EN : locale;
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

    public static DurationRange rangeOf(String expression, Locale locale) {
        Locale safeLocale = locale == null ? Locale.EN : locale;
        return rangeOf(expression, safeLocale.code());
    }

    public static List<ExpressionCandidate> extractExpressions(String input, Locale locale) {
        Objects.requireNonNull(input, "input must not be null");
        Locale safeLocale = locale == null ? Locale.EN : locale;

        List<ExpressionCandidate> raw = new ArrayList<>();
        for (Pattern p : localePatterns(safeLocale)) {
            Matcher m = p.matcher(input);
            while (m.find()) {
                String text = m.group().trim();
                if (text.isEmpty()) {
                    continue;
                }
                try {
                    rangeOf(text, safeLocale);
                    raw.add(new ExpressionCandidate(m.start(), m.end(), text));
                } catch (IllegalArgumentException ignored) {
                    // keep only parseable range expressions.
                }
            }
        }

        raw.sort((a, b) -> {
            int lenA = a.end() - a.start();
            int lenB = b.end() - b.start();
            if (lenA != lenB) {
                return Integer.compare(lenB, lenA);
            }
            return Integer.compare(a.start(), b.start());
        });

        List<ExpressionCandidate> selected = new ArrayList<>();
        for (ExpressionCandidate c : raw) {
            boolean overlap = false;
            for (ExpressionCandidate s : selected) {
                if (s.start() < c.end() && c.start() < s.end()) {
                    overlap = true;
                    break;
                }
            }
            if (!overlap) {
                selected.add(c);
            }
        }
        selected.sort((a, b) -> Integer.compare(a.start(), b.start()));
        return selected;
    }

    public static DurationRange rangeOf(String expression, String locale) {
        Objects.requireNonNull(expression, "expression must not be null");
        String safeLocale = locale == null ? "en" : locale;

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment expressionPtr = arena.allocateFrom(expression);
            MemorySegment localePtr = arena.allocateFrom(safeLocale);
            MemorySegment outStart = arena.allocate(JAVA_LONG);
            MemorySegment outEnd = arena.allocate(JAVA_LONG);

            boolean ok = (boolean) RANGE_OF.invoke(
                    expressionPtr,
                    localePtr,
                    outStart,
                    outEnd
            );
            if (!ok) {
                throw new IllegalArgumentException("Failed to parse OOTD expression");
            }

            long startSeconds = outStart.get(JAVA_LONG, 0);
            long endSeconds = outEnd.get(JAVA_LONG, 0);
            return new DurationRange(
                    Duration.ofSeconds(startSeconds),
                    Duration.ofSeconds(endSeconds)
            );
        } catch (IllegalArgumentException e) {
            throw e;
        } catch (Throwable t) {
            throw new IllegalArgumentException("Failed to parse OOTD expression", t);
        }
    }

    private static TimestampRange resolveDurationRangeAtRfc3339(
            DurationRange range,
            String anchorRfc3339
    ) {
        Objects.requireNonNull(range, "range must not be null");
        Objects.requireNonNull(anchorRfc3339, "anchorRfc3339 must not be null");

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment anchorPtr = arena.allocateFrom(anchorRfc3339);
            MemorySegment outStartRfc3339Ptr = arena.allocate(ADDRESS);
            MemorySegment outEndRfc3339Ptr = arena.allocate(ADDRESS);

            boolean ok = (boolean) RESOLVE_DURATION_RANGE_AT_RFC3339.invoke(
                    range.start().getSeconds(),
                    range.end().getSeconds(),
                    anchorPtr,
                    outStartRfc3339Ptr,
                    outEndRfc3339Ptr
            );
            if (!ok) {
                throw new IllegalArgumentException("Failed to resolve OOTD duration range");
            }

            MemorySegment rawStart = outStartRfc3339Ptr.get(ADDRESS, 0);
            MemorySegment rawEnd = outEndRfc3339Ptr.get(ADDRESS, 0);
            String startRfc3339 = consumeNativeString(rawStart);
            String endRfc3339 = consumeNativeString(rawEnd);

            return new TimestampRange(
                    OffsetDateTime.parse(startRfc3339),
                    OffsetDateTime.parse(endRfc3339)
            );
        } catch (IllegalArgumentException e) {
            throw e;
        } catch (Throwable t) {
            throw new IllegalArgumentException("Failed to resolve OOTD duration range", t);
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

    private static List<Pattern> localePatterns(Locale locale) {
        if (locale == Locale.KO) {
            return List.of(
                    Pattern.compile("([0-9]{1,3}|[가-힣]{1,8})\\s*(년|달|주|일|시간|분|초)(\\s*반)?\\s*(전|후)"),
                    Pattern.compile("(어제|오늘|내일)\\s*(새벽|아침|낮|저녁|밤)")
            );
        }
        return List.of(
                Pattern.compile("(?i)\\b(a|an|\\d+)\\s+(year|years|month|months|week|weeks|day|days|hour|hours|minute|minutes|second|seconds)(\\s+and\\s+a\\s+half)?\\s+(ago|later)\\b"),
                Pattern.compile("(?i)\\b(yesterday|this|tomorrow)\\s+(dawn|morning|afternoon|evening|night)\\b"),
                Pattern.compile("(?i)\\b(last night|earlier tonight|tonight)\\b")
        );
    }
}
