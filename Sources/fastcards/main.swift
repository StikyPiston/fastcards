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
