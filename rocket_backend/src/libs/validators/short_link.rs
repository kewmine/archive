use {
    rocket::State,
    rocket_db_pools::mongodb::{bson::doc, Collection},
};
use crate::{
    libs::unique_string,
    models::links::Link,
};

// update the short id with a new one if exists in db
pub async fn short_id_valid(doc: &mut Link, db_session: &State<Collection<Link>>) {
    let short_id_valid = short_id_exists(&doc, db_session).await;

    while short_id_valid == false {
        let new_id = unique_string::link_short_id();
        doc.link_id = new_id.to_string();
    }
}

// check if the id exists in db
async fn short_id_exists(doc: &Link,db_session: &State<Collection<Link>>) -> bool {
    let filter = doc! {"link_short_id": &doc.link_id};
    let fetcher = db_session
        .find_one(filter, None).await
        .expect("exception while trying to fetch short_id");

    match fetcher {
        Some(_doc) => { false }
        None => { true }
    }
}
