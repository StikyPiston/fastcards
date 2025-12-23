import Foundation

struct XPStore {
    static let xpURL = dataURL.appendingPathComponent("xp.json")

    static func load() -> Int {
        guard
            let data = try? Data(contentsOf: xpURL),
            let json = try? JSONSerialization.jsonObject(with: data) as? [String: Int],
            let xp   = json["xp"]
        else {
            return 0
        }

        return xp
    }

    static func save(_ xp: Int) {
        let json = ["xp": xp]
        guard let data = try? JSONSerialization.data(
            withJSONObject: json,
            options: .prettyPrinted
        ) else { return }

        try? data.write(to: xpURL, options: .atomic)
    }
}
