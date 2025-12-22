import Foundation

// MARK: Card
struct Card: Codable, Identifiable {
    let id: UUID
    var front: String
    var back: String
    var state: Int
    var lastReviewed: Date?

    init(
        front: String,
        back: String,
        state: Int = 1,
        lastReviewed: Date? = nil
    ) {
        self.id           = UUID()
        self.front        = front
        self.back         = back
        self.state        = state
        self.lastReviewed = lastReviewed
    }

    enum CodingKeys: String, CodingKey {
        case id
        case front
        case back
        case state
        case lastReviewed = "last_reviewed"
    }
}

// MARK: Deck
struct Deck: Codable, Identifiable {
    let id: UUID
    var name: String
    var cards: [Card]

    init(name: String, cards: [Card] = []) {
        self.id    = UUID()
        self.name  = name
        self.cards = cards
    }
}
