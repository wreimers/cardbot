use crate::card::{Card, CardRaw};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    pub public_id: String,
    pub cards: Vec<Card>,
    pub name: String,
}

impl From<DeckAPIResult> for Deck {
    fn from(deck: DeckAPIResult) -> Self {
        Deck {
            public_id: deck.public_id.to_owned(),
            cards: deck
                .mainboard
                .into_iter()
                .map(|(_, cardraw)| Card::from(cardraw))
                .collect(),
            name: deck.name.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckAPIResult {
    pub name: String,
    pub description: String,
    pub public_url: String,
    pub public_id: String,
    pub mainboard: HashMap<String, CardRaw>,
}
