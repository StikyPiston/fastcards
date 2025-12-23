import Foundation

// MARK: initialise variables
let fm       = FileManager.default
let homeURL  = fm.homeDirectoryForCurrentUser
let baseURL  = homeURL.appendingPathComponent(".fastcards")
let decksURL = baseURL.appendingPathComponent("decks")
let dataURL  = baseURL.appendingPathComponent("data")

do {
    try fm.createDirectory(at: decksURL, withIntermediateDirectories: true)
    try fm.createDirectory(at: dataURL, withIntermediateDirectories: true)
} catch {
    print(" Failed to create directories: \(error)")
    exit(1)
}

// MARK: Entrypoint
let args = CommandLine.arguments

if args.count > 1 {
    let action = args[1]

    switch action {
        case "list":
            let decks = DeckStore.listDecks()
            if decks.isEmpty {
                print("󰘸 No decks found")
            } else {
                decks.forEach{ print($0) }
            }
        case "createdeck":
            guard args.count > 2 else {
                print("Usage: fastcards createdeck <name>")
                exit(1)
            }

            let name = args[2]

            if DeckStore.listDecks().contains(name) {
                print("󰘸 Deck \(name) already exists!")
                exit(1)
            }
            
            let deck = Deck(name: name)

            do {
                try DeckStore.save(deck: deck)
                print("󰘸 Deck created: \(name)")
            } catch {
                print(" Failed to create deck: \(error)")
            }
        case "addcard":
            guard args.count > 4 else {
                print("Usage: fastcards addcard <deckname> <front> <back>")
                exit(1)
            }

            let deckName = args[2]
            let front    = args[3]
            let back     = args[4]

            do {
                var deck = try DeckStore.load(name: deckName)
                let card = Card(front: front, back: back)

                deck.cards.append(card)
                try DeckStore.save(deck: deck)
                print("󱇿 Added card to \(deck.name)")
            } catch {
                print(" Error adding card to deck: \(error)")
                exit(1)
            }
        case "amount":
            let decks = DeckStore.loadAllDecks()
            let due   = decks.flatMap { $0.cards }
                .filter { Scheduler.isDue($0) }
                .count
            print("󰘸 Flashcards due: \(due)")
        default:
            print("Usage: fastcards <action> <arguments>")
            print("> list                          - List available decks")
            print("> createdeck <name>             - Create a new flashcard deck")
            print("> addcard <deck> <front> <back> - Add card to deck")
            print("> amount                        - Print amount of flashcards due")
    }
} else {
    print("Usage: fastcards <action> <arguments>")
    print("> list              - List available decks")
    print("> createdeck <name> - Create a new flashcard deck")
    print("> addcard <deck> <front> <back> - Add card to deck")
    print("> amount                        - Print amount of flashcards due")
}
