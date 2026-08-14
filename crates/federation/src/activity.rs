use activitystreams_kinds::activity::{AcceptType, CreateType, FollowType, LikeType, UndoType};
use serde::{Deserialize, Serialize};
use url::Url;

/// ActivityPub Create activity wrapping an object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActivity {
    #[serde(rename = "type")]
    pub kind: CreateType,
    pub id: Url,
    pub actor: Url,
    pub object: serde_json::Value,
    #[serde(default)]
    pub to: Vec<serde_json::Value>,
    #[serde(default)]
    pub cc: Vec<serde_json::Value>,
}

/// ActivityPub Follow activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowActivity {
    #[serde(rename = "type")]
    pub kind: FollowType,
    pub id: Url,
    pub actor: Url,
    pub object: Url,
}

/// ActivityPub Accept activity (accepting a follow)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptActivity {
    #[serde(rename = "type")]
    pub kind: AcceptType,
    pub id: Url,
    pub actor: Url,
    pub object: FollowActivity,
}

/// ActivityPub Undo activity (undoing a follow)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoActivity {
    #[serde(rename = "type")]
    pub kind: UndoType,
    pub id: Url,
    pub actor: Url,
    pub object: FollowActivity,
}

/// ActivityPub Like activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeActivity {
    #[serde(rename = "type")]
    pub kind: LikeType,
    pub id: Url,
    pub actor: Url,
    pub object: Url,
}

/// ActivityPub Undo activity (undoing a like)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoLikeActivity {
    #[serde(rename = "type")]
    pub kind: UndoType,
    pub id: Url,
    pub actor: Url,
    pub object: LikeActivity,
}
