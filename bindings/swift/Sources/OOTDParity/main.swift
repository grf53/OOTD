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
