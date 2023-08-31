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
    fn from(cardraw: CardRaw) -> Self {
        Card {
            name: cardraw.card["name"].as_str().unwrap().to_string(),
            scryfall_id: cardraw.card["scryfall_id"].as_str().unwrap().to_string(),
        }
    }
}

impl From<&CardRaw> for Card {
    fn from(cardraw: &CardRaw) -> Self {
        Card {
            name: cardraw.card["name"].as_str().unwrap().to_string(),
            scryfall_id: cardraw.card["scryfall_id"].as_str().unwrap().to_string(),
        }
    }
}
