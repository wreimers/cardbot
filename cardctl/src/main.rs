use clap::Parser;
mod interface;
use interface::{get_moxfield_deck, get_scryfall_card};

/// cardbot cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// moxfield deck id
    #[arg(short, long)]
    moxfield_id: Option<String>,
    /// scryfall card id
    #[arg(short, long)]
    scryfall_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    if args.moxfield_id.is_some() {
        match get_moxfield_deck(args.moxfield_id.unwrap().clone()).await {
            None => {
                println!("No Moxfield deck found with that ID.");
            }
            Some(deck) => {
                println!("{}", serde_json::to_string(&deck).unwrap());
            }
        }
    }

    if args.scryfall_id.is_some() {
        match get_scryfall_card(args.scryfall_id.unwrap().clone()).await {
            None => {
                println!("No Scryfall card found with that ID.");
            }
            Some(deck) => {
                println!("{}", serde_json::to_string(&deck).unwrap());
            }
        }
    }

    Ok(())
}
