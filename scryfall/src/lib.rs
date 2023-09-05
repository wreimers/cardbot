use serde::{Deserialize, Serialize};
use std::fmt;

pub type ScryfallCard = Card;
pub type ScryfallError = Error;

const API_ENDPOINT: &str = "https://api.scryfall.com/cards";

pub async fn card_with_id(scryfall_id: String) -> Result<Option<ScryfallCard>, ScryfallError> {
    let resource = format!("{}/{}", API_ENDPOINT, scryfall_id);
    let api_result = api_get(resource).await?;
    if !api_result.is_success {
        return Ok(None);
    }
    match serde_json::from_str::<CardAPIResult>(api_result.body.as_str()) {
        Ok(result) => Ok(Some(ScryfallCard::from(result))),
        Err(e) => Err(ScryfallError::from(e)),
    }
}

pub async fn api_get(resource: String) -> Result<APIResult, ScryfallError> {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub scryfall_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAPIResult {
    pub name: String,
    pub id: String,
}

impl From<CardAPIResult> for Card {
    fn from(card: CardAPIResult) -> Self {
        Card {
            scryfall_id: card.id.to_owned(),
            name: card.name.to_owned(),
        }
    }
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

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Scryfall(APIError),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIError {
    pub description: String,
}

macro_rules! impl_display_error {
    ($ekind:ident) => {
        impl fmt::Display for $ekind {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
impl_display_error!(Card);
impl_display_error!(Error);
impl_display_error!(APIError);

macro_rules! impl_from_error {
    ($etype:ty, $ewrap:ident) => {
        impl From<$etype> for Error {
            fn from(error: $etype) -> Self {
                Self::$ewrap(error)
            }
        }
    };
}
impl_from_error!(reqwest::Error, Reqwest);
impl_from_error!(serde_json::Error, SerdeJson);
impl_from_error!(APIError, Scryfall);
