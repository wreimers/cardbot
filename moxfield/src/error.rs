use crate::card::Card;
use crate::card::CardRaw;
use crate::deck::Deck;
use crate::deck::DeckAPIResult;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Moxfield(APIError),
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
impl_display_error!(Error);
impl_display_error!(Card);
impl_display_error!(Deck);
impl_display_error!(CardRaw);
impl_display_error!(DeckAPIResult);
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
impl_from_error!(APIError, Moxfield);
