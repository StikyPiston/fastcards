import Foundation

struct DeckStore {
    static let encoder: JSONEncoder = {
        let enc = JSONEncoder()
        enc.outputFormatting     = .prettyPrinted
        enc.dateEncodingStrategy = .iso8601
        return enc
    }()

    static let decoder: JSONDecoder = {
        let dec                  = JSONDecoder()
        dec.dateDecodingStrategy = .iso8601
        return dec
    }()

    static func deckURL(name: String) -> URL {
        decksURL.appendingPathComponent("\(name).json")
    }

    static func save(deck: Deck) throws {
        let url  = deckURL(name: deck.name)
        let data = try encoder.encode(deck)
        try data.write(to: url, options: .atomic)
    }

    static func load(name: String) throws -> Deck {
        let url  = deckURL(name: name)
        let data = try Data(contentsOf: url)
        return try decoder.decode(Deck.self, from: data)
    }

    static func listDecks() -> [String] {
        guard let files = try? FileManager.default.contentsOfDirectory(at: decksURL, includingPropertiesForKeys: nil) else { return [] }

        return files
                .filter { $0.pathExtension == "json" }
                .map    { $0.deletingPathExtension().lastPathComponent }
                .sorted()
    }
}
