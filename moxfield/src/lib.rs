use serde::{Deserialize, Serialize};

mod card;
mod deck;
mod error;

pub type MoxfieldCard = card::Card;
pub type MoxfieldDeck = deck::Deck;
pub type MoxfieldError = error::Error;

use deck::DeckAPIResult;

const DECK_API_ENDPOINT: &str = "https://api.moxfield.com/v2/decks/all";

pub async fn deck_with_public_id(id: &str) -> Result<Option<MoxfieldDeck>, MoxfieldError> {
    let resource = format!("{}/{}", DECK_API_ENDPOINT, id);
    let api_result = api_get(resource).await?;
    if !api_result.is_success {
        return Ok(None);
    }
    match serde_json::from_str::<DeckAPIResult>(api_result.body.as_str()) {
        Ok(result) => Ok(Some(MoxfieldDeck::from(result))),
        Err(e) => Err(MoxfieldError::from(e)),
    }
}

pub async fn api_get(resource: String) -> Result<APIResult, MoxfieldError> {
    let response = reqwest::get(&resource).await?;
    Ok(APIResult {
        method: APIMethod::GET,
        resource,
        status: response.status().to_string(),
        is_success: response.status().is_success(),
        status_code: response.status().as_u16(),
        body: response.text().await.unwrap(),
    })
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResult {
    method: APIMethod,
    resource: String,
    is_success: bool,
    status_code: u16,
    status: String,
    body: String,
}

pub type APIMethod = AllowedAPIMethods;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AllowedAPIMethods {
    #[default]
    GET,
}
