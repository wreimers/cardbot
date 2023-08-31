use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub scryfall_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRaw {
    pub card: HashMap<String, Value>,
}

impl From<CardRaw> for Card {
    fn from(card: CardRaw) -> Self {
        Card {
            name: card.card["name"].to_string(),
            scryfall_id: card.card["scryfall_id"].to_string(),
        }
    }
}
