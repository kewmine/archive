use ::serde::{Serialize, Deserialize};
use ::rocket_db_pools::mongodb::bson::{
    oid::ObjectId,
    DateTime
};

// Document structure for Collection:Links
#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub oid: Option<ObjectId>,
    pub created_on: DateTime,
    pub long_uri: String,
    pub link_domain: String,
    pub link_id: String
}
