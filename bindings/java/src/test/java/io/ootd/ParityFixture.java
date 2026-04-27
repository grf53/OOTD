package io.ootd;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

final class ParityFixture {
    record BetweenCase(
            String name,
            String start,
            String end,
            String locale,
            boolean useNativeKoNumber,
            String expected
    ) {}

    record DurationCase(
            String name,
            long seconds,
            boolean isFuture,
            String locale,
            boolean useNativeKoNumber,
            String expected,
            String expectedError
    ) {}

    final List<BetweenCase> betweenCases;
    final List<DurationCase> durationCases;

    private ParityFixture(List<BetweenCase> betweenCases, List<DurationCase> durationCases) {
        this.betweenCases = betweenCases;
        this.durationCases = durationCases;
    }

    static ParityFixture load() {
        Path path = Path.of("..", "..", "tests", "parity_cases.json");
        try {
            String json = Files.readString(path, StandardCharsets.UTF_8);
            return parse(json);
        } catch (IOException e) {
            throw new IllegalStateException("Failed to read parity fixture: " + path.toAbsolutePath(), e);
        }
    }

    private static ParityFixture parse(String json) {
        String betweenArray = findArray(json, "between_cases");
        String durationArray = findArray(json, "duration_cases");

        List<BetweenCase> between = new ArrayList<>();
        for (String obj : splitObjects(betweenArray)) {
            between.add(new BetweenCase(
                    requiredString(obj, "name"),
                    requiredString(obj, "start"),
                    requiredString(obj, "end"),
                    requiredString(obj, "locale"),
                    optionalBoolean(obj, "use_native_ko_number", false),
                    requiredString(obj, "expected")
            ));
        }

        List<DurationCase> duration = new ArrayList<>();
        for (String obj : splitObjects(durationArray)) {
            duration.add(new DurationCase(
                    requiredString(obj, "name"),
                    requiredLong(obj, "seconds"),
                    requiredBoolean(obj, "is_future"),
                    requiredString(obj, "locale"),
                    optionalBoolean(obj, "use_native_ko_number", false),
                    optionalString(obj, "expected"),
                    optionalString(obj, "expected_error")
            ));
        }

        return new ParityFixture(between, duration);
    }

    private static String findArray(String json, String key) {
        String needle = "\"" + key + "\"";
        int keyPos = json.indexOf(needle);
        if (keyPos < 0) {
            throw new IllegalStateException("Missing key in parity fixture: " + key);
        }

        int start = json.indexOf('[', keyPos);
        if (start < 0) {
            throw new IllegalStateException("Missing array start for key: " + key);
        }

        int depth = 0;
        for (int i = start; i < json.length(); i++) {
            char ch = json.charAt(i);
            if (ch == '[') {
                depth++;
            } else if (ch == ']') {
                depth--;
                if (depth == 0) {
                    return json.substring(start + 1, i);
                }
            }
        }

        throw new IllegalStateException("Unclosed array for key: " + key);
    }

    private static List<String> splitObjects(String arrayContent) {
        List<String> out = new ArrayList<>();
        int depth = 0;
        int start = -1;
        for (int i = 0; i < arrayContent.length(); i++) {
            char ch = arrayContent.charAt(i);
            if (ch == '{') {
                if (depth == 0) {
                    start = i;
                }
                depth++;
            } else if (ch == '}') {
                depth--;
                if (depth == 0 && start >= 0) {
                    out.add(arrayContent.substring(start, i + 1));
                    start = -1;
                }
            }
        }

        return out;
    }

    private static String requiredString(String object, String key) {
        String value = optionalString(object, key);
        if (value == null) {
            throw new IllegalStateException("Missing required string key: " + key + " in " + object);
        }
        return value;
    }

    private static String optionalString(String object, String key) {
        Matcher m = Pattern.compile("\"" + Pattern.quote(key) + "\"\\s*:\\s*\"([^\"]*)\"").matcher(object);
        if (m.find()) {
            return m.group(1);
        }
        return null;
    }

    private static long requiredLong(String object, String key) {
        Matcher m = Pattern.compile("\"" + Pattern.quote(key) + "\"\\s*:\\s*(-?\\d+)").matcher(object);
        if (!m.find()) {
            throw new IllegalStateException("Missing required long key: " + key + " in " + object);
        }
        return Long.parseLong(m.group(1));
    }

    private static boolean requiredBoolean(String object, String key) {
        Matcher m = Pattern.compile("\"" + Pattern.quote(key) + "\"\\s*:\\s*(true|false)").matcher(object);
        if (!m.find()) {
            throw new IllegalStateException("Missing required boolean key: " + key + " in " + object);
        }
        return Boolean.parseBoolean(m.group(1));
    }

    private static boolean optionalBoolean(String object, String key, boolean defaultValue) {
        Matcher m = Pattern.compile("\"" + Pattern.quote(key) + "\"\\s*:\\s*(true|false)").matcher(object);
        if (m.find()) {
            return Boolean.parseBoolean(m.group(1));
        }
        return defaultValue;
    }
}
