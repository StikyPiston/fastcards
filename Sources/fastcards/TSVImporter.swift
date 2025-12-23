import Foundation

enum TSVImporter {
    static func `import`(path: String) throws {
        let content = try String(contentsOfFile: path, encoding: .utf8)
        let lines   = content.split(whereSeparator:  \.isNewline)

        var decksByName: [String: Deck] = [:]

        for name in DeckStore.listDecks() {
            decksByName[name] = try DeckStore.load(name: name)
        }

        var importedCount: [String: Int] = [:]

        for (i, line) in lines.enumerated() {
            let cols = line.split(separator: "\t", omittingEmptySubsequences: false)
            guard cols.count == 3 else {
                print("Skipping malformed line \(i + 1)")
                continue
            }

            let deckName = String(cols[0])
            let front    = String(cols[1])
            let back     = String(cols[2])

            if decksByName[deckName] == nil {
                decksByName[deckName] = Deck(name: deckName)
            }

            let card = Card(front: front, back: back)
            decksByName[deckName]?.cards.append(card)
            importedCount[deckName, default: 0] += 1
        }

        for deck in decksByName.values {
            try DeckStore.save(deck: deck)
        }

        print("󰋺 Import complete:")
        for (deck, count) in importedCount.sorted(by: { $0.key < $1.key }) {
            print("> \(deck): \(count) cards")
        }
    }
}
