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
    fn from(rawcard: CardRaw) -> Self {
        Card {
            name: rawcard.card["name"].to_string(),
            scryfall_id: rawcard.card["scryfall_id"].to_string(),
        }
    }
}

impl From<&CardRaw> for Card {
    fn from(rawcard_handle: &CardRaw) -> Self {
        Card {
            name: rawcard_handle.card["name"].as_str().unwrap().to_string(),
            scryfall_id: rawcard_handle.card["scryfall_id"]
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
