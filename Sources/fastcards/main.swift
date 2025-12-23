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
        default:
            print("Usage: fastcards <action> <arguments>")
            print("> list - Lists available decks")
    }
}
