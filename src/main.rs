use clap::{Parser, Subcommand};

mod models;
mod scheduler;
mod storage;

mod add_cmd;
mod amount_cmd;
mod archive_list_cmd;
mod create_cmd;
mod import_cmd;
mod list_cmd;
mod xp_cmd;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "fastcards")]
#[command(version = &VERSION)]
#[command(about = "A simple spaced-repetition flashcard study tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a card to a deck
    Add {
        deck: String,
        front: String,
        back: String,
    },
    /// Print the amount of cards currently due
    Amount { deck: Option<String> },
    /// Manage the Deck Archive
    Archive {
        #[command(subcommand)]
        command: ArchiveCommands,
    },
    /// Create a new deck
    Create { name: String },
    /// Import flashcards from a .tsv file
    Import { path: String },
    /// List all decks and the amount of cards in them
    List,
    /// Study due cards
    Study { deck: Option<String> },
    /// Show XP amount
    Xp,
}

#[derive(Subcommand)]
enum ArchiveCommands {
    /// Add a deck to the Deck Archive
    Add { deck: String },
    /// List the contents of the Deck Archive
    List,
    /// Remove a deck from the Deck Archive
    Remove { deck: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { deck, front, back } => {
            add_cmd::add(deck.to_string(), front.to_string(), back.to_string())
        }
        Commands::Amount { deck } => amount_cmd::amount(deck.clone()),
        Commands::Archive { command } => match command {
            ArchiveCommands::Add { deck } => todo!(),
            ArchiveCommands::List {} => archive_list_cmd::list(),
            ArchiveCommands::Remove { deck } => todo!(),
        },
        Commands::Create { name } => create_cmd::create(name.clone()),
        Commands::Import { path } => import_cmd::import(path.clone()),
        Commands::List {} => list_cmd::list(),
        Commands::Study { deck } => todo!(),
        Commands::Xp {} => xp_cmd::xp(),
    }
}
