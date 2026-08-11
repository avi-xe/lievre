use activitypub_federation::protocol::public_key::PublicKey;
use activitystreams_kinds::actor::PersonType;
use serde::{Deserialize, Serialize};
use url::Url;

/// ActivityPub Person actor (fedisport-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    pub kind: PersonType,
    pub id: Url,
    pub preferred_username: String,
    #[serde(default)]
    pub name: Option<String>,
    pub inbox: Url,
    pub outbox: Url,
    #[serde(default)]
    pub following: Option<Url>,
    #[serde(default)]
    pub followers: Option<Url>,
    pub public_key: PublicKey,
    #[serde(default)]
    pub icon: Option<PersonIcon>,
}

/// Person icon (avatar)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonIcon {
    #[serde(rename = "type")]
    pub kind: String,
    pub media_type: String,
    pub url: Url,
}
