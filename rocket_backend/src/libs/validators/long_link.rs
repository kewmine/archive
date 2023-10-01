use {
    rocket::State,
    rocket_db_pools::mongodb::{bson::doc, Collection},
};
use crate::models::links::Link;

pub async fn long_link_exists(
    long_uri: &String,
    db_session: &State<Collection<Link>>,
) -> Option<String> {
    let filter = doc! {"link_long_uri": long_uri};
    let fetcher = db_session
        .find_one(filter, None).await
        .expect("exception while trying to fetch long_uri");

    match fetcher {
        Some(doc) => {
            let short_uri = format!("{}/{}", doc.link_domain, doc.link_id);
            Some(short_uri)
        }
        None => { None }
    }
}
