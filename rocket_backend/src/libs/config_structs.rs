use rocket::serde::Deserialize;

// db handle
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DbConfig {
    pub conn_uri: String,
    pub db_name: String,
    pub db_coll: String,
    pub short_id: String,
    pub long_uri: String,
}

// common template vars
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TemplateConfig {
    pub project_name: String,
    pub description: String,
    pub author: String,
}
