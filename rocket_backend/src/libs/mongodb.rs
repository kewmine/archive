use rocket_db_pools::{mongodb, mongodb::Client};

pub async fn mongodb_session(connection_uri: &str) -> Client {
    let mdb_client = mongodb::Client::with_uri_str(connection_uri).await.expect("failed to make a session");
    mdb_client
}
