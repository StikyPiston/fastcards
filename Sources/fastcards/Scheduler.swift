import Foundation

enum Scheduler {
    static let intervals: [Int: Int] = [
        1: 1,
        2: 2,
        3: 4,
        4: 9,
        5: 14
    ]

    static let maxState = 5

    static func isDue(_ card: Card, today: Date = Date()) -> Bool {
        guard let last = card.lastReviewed else {
            return true
        }

        let days = Calendar.current.dateComponents(
            [.day],
            from: last,
            to: today
        ).day ?? 0

        let interval = intervals[card.state] ?? 1
        return days >= interval
    }
}
