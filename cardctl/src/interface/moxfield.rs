use ::moxfield::{deck_with_public_id, MoxfieldDeck};

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
