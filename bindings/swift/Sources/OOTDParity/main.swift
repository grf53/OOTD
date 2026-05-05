import Foundation
import OOTD

private struct ParityFixture: Decodable {
    struct BetweenCase: Decodable {
        let name: String
        let start: String
        let end: String
        let locale: String
        let useNativeKoNumber: Bool?
        let expected: String

        private enum CodingKeys: String, CodingKey {
            case name
            case start
            case end
            case locale
            case useNativeKoNumber = "use_native_ko_number"
            case expected
        }
    }

    struct DurationCase: Decodable {
        let name: String
        let seconds: Int64
        let isFuture: Bool
        let locale: String
        let useNativeKoNumber: Bool?
        let expected: String?
        let expectedError: String?

        private enum CodingKeys: String, CodingKey {
            case name
            case seconds
            case isFuture = "is_future"
            case locale
            case useNativeKoNumber = "use_native_ko_number"
            case expected
            case expectedError = "expected_error"
        }
    }

    let betweenCases: [BetweenCase]
    let durationCases: [DurationCase]

    private enum CodingKeys: String, CodingKey {
        case betweenCases = "between_cases"
        case durationCases = "duration_cases"
    }
}

private enum RunnerError: Error, CustomStringConvertible {
    case fixtureLoad(String)
    case failures([String])

    var description: String {
        switch self {
        case let .fixtureLoad(message):
            return message
        case let .failures(lines):
            return lines.joined(separator: "\n")
        }
    }
}

@main
struct OOTDParityRunner {
    static func main() {
        do {
            try run()
            print("OOTD Swift parity passed")
        } catch {
            fputs("OOTD Swift parity failed: \(error)\n", stderr)
            exit(1)
        }
    }

    private static func run() throws {
        let fixturePath = ProcessInfo.processInfo.environment["OOTD_PARITY_FIXTURE"]
            ?? "../../tests/parity_cases.json"

        let url = URL(fileURLWithPath: fixturePath)
        guard let data = try? Data(contentsOf: url) else {
            throw RunnerError.fixtureLoad("failed to read fixture at: \(url.path)")
        }

        let fixture: ParityFixture
        do {
            fixture = try JSONDecoder().decode(ParityFixture.self, from: data)
        } catch {
            throw RunnerError.fixtureLoad("failed to decode parity fixture: \(error)")
        }

        var failures: [String] = []

        for c in fixture.betweenCases {
            let locale = mapLocale(c.locale)
            do {
                let out = try OOTD.between(
                    startRFC3339: c.start,
                    endRFC3339: c.end,
                    locale: locale,
                    useNativeKoNumber: c.useNativeKoNumber ?? false
                )
                if out != c.expected {
                    failures.append("between parity mismatch (\(c.name)): \(out) != \(c.expected)")
                }
            } catch {
                failures.append("between parity threw (\(c.name)): \(error)")
            }
        }

        for c in fixture.durationCases {
            let locale = mapLocale(c.locale)
            if let expectedError = c.expectedError {
                do {
                    _ = try OOTD.fromDuration(
                        seconds: c.seconds,
                        isFuture: c.isFuture,
                        locale: locale,
                        useNativeKoNumber: c.useNativeKoNumber ?? false
                    )
                    failures.append("duration error case did not fail (\(c.name))")
                } catch {
                    if !String(describing: error).contains(expectedError) {
                        failures.append(
                            "duration error mismatch (\(c.name)): \(error) does not contain \(expectedError)"
                        )
                    }
                }
                continue
            }

            do {
                let out = try OOTD.fromDuration(
                    seconds: c.seconds,
                    isFuture: c.isFuture,
                    locale: locale,
                    useNativeKoNumber: c.useNativeKoNumber ?? false
                )
                if out != c.expected {
                    failures.append("duration parity mismatch (\(c.name)): \(out) != \(c.expected ?? "<nil>")")
                }
            } catch {
                failures.append("duration parity threw (\(c.name)): \(error)")
            }
        }

        do {
            let range = try OOTD.rangeOf(expression: "두 달 전", locale: .ko)
            if range.start != .seconds(Double(-6_047_999)) || range.end != .seconds(Double(-4_752_000)) {
                failures.append("rangeOf mismatch: \(range)")
            }
            if range.start.components.seconds != -6_047_999 || range.end.components.seconds != -4_752_000 {
                failures.append(
                    "rangeOf seconds mismatch: (\(range.start.components.seconds), \(range.end.components.seconds))"
                )
            }

            let resolved = try range.resolveAt("2026-04-29T12:00:00+09:00")
            let formatter = ISO8601DateFormatter()
            formatter.formatOptions = [.withInternetDateTime]
            guard
                let expectedStart = formatter.date(from: "2026-02-18T12:00:01+09:00"),
                let expectedEnd = formatter.date(from: "2026-03-05T12:00:00+09:00")
            else {
                failures.append("failed to build expected timestamp for range resolve case")
                return
            }
            if resolved.start != expectedStart || resolved.end != expectedEnd {
                failures.append("range resolve mismatch: \(resolved)")
            }
            let rfcFormatter = ISO8601DateFormatter()
            rfcFormatter.formatOptions = [.withInternetDateTime]
            rfcFormatter.timeZone = TimeZone(secondsFromGMT: 9 * 3600)!
            let resolvedStart = rfcFormatter.string(from: resolved.start)
            let resolvedEnd = rfcFormatter.string(from: resolved.end)
            if resolvedStart != "2026-02-18T12:00:01+09:00" || resolvedEnd != "2026-03-05T12:00:00+09:00" {
                failures.append(
                    "range resolve RFC3339 mismatch: (\(resolvedStart), \(resolvedEnd))"
                )
            }
        } catch {
            failures.append("rangeOf/resolveAt threw: \(error)")
        }

        if !failures.isEmpty {
            throw RunnerError.failures(failures)
        }
    }

    private static func mapLocale(_ locale: String) -> OOTDLocale {
        switch locale {
        case "ko":
            return .ko
        default:
            return .en
        }
    }
}
