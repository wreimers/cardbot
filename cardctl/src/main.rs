use clap::Parser;
mod interface;
use interface::get_moxfield_deck;
use serde_json;

/// cardbot cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// moxfield deck id
    #[arg(short, long)]
    moxfield_id: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    if !args.moxfield_id.is_empty() {
        match get_moxfield_deck(args.moxfield_id.clone()).await {
            None => {
                println!("No Moxfield deck found with that ID.");
            }
            Some(deck) => {
                println!("{}", serde_json::to_string(&deck).unwrap());
            }
        }
    }

    Ok(())
}
