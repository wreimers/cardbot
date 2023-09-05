use ::moxfield::{deck_with_public_id, MoxfieldDeck};
use ::scryfall::{card_with_id, ScryfallCard};

pub async fn get_moxfield_deck(with_id: String) -> Option<MoxfieldDeck> {
    let deck_result = deck_with_public_id(&with_id).await;
    match deck_result {
        Ok(Some(deck)) => Some(deck),
        Ok(None) => None,
        Err(e) => {
            println!("Some error occurred: {:?}", e);
            None
        }
    }
}

pub async fn get_scryfall_card(with_id: String) -> Option<ScryfallCard> {
    let card_result = card_with_id(with_id).await;
    match card_result {
        Ok(Some(card)) => Some(card),
        Ok(None) => None,
        Err(e) => {
            println!("Some error occurred: {:?}", e);
            None
        }
    }
}

// todo: backend trait
