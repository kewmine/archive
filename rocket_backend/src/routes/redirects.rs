use rocket::{State, response::Redirect};
use rocket_db_pools::mongodb::{Collection, bson::doc};
use crate::{
    models::links::Link,
    libs::config_structs::DbConfig,
};

#[get("/<short_id>")]
pub async fn redirect_to_long(
    short_id: String,
    links_coll: &State<Collection<Link>>,
    db_session: &State<DbConfig>) -> Redirect {

    // query to find if a record with the requested short id exists
    let fetcher = links_coll.find_one(
        doc! {
            &db_session.short_id:short_id
        }, None).await
        .expect("error while fetching a link");

    // redirect to the linked uri if found and if not, to 404 page
    match fetcher {
        Some(doc) => {
            Redirect::found(format!("{}", doc.long_uri))
        }
        None => {
            Redirect::to(uri!("http://localhost:8080/404"))
        }
    }
}
