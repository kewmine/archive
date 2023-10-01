use rocket::form::{Form, Lenient};
use rocket::http::uri::Uri;
use rocket::State;
use rocket_db_pools::{mongodb::bson::DateTime};
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::Collection;
use rocket_dyn_templates::{context, Template};

use crate::{
    libs::{unique_string},
    models::links::Link};
use crate::libs::validators;

#[derive(FromForm)]
pub struct LongUriForm {
    link_long_uri: String,
}

// page to create link
#[get("/create")]
pub fn create_link_page() -> Template {
    Template::render(
        "pages/link_create",
        context! {},
    )
}

// post request handler to generate a shortened link.
#[post("/create", data = "<post_data>")]
pub async fn create_link_request(
    post_data: Form<Lenient<LongUriForm>>,
    links_coll: &State<Collection<Link>>,
) -> String {
    let long_uri = &post_data.link_long_uri;

    // if uri submitted is invalid return message or if valid do nothing and proceed
    let parse_uri = Uri::parse_any(long_uri);
    match parse_uri {
        Ok(uri) => drop(uri),
        Err(_error) => {
            let msg = String::from("uri submitted is invalid.");
            return msg;
        }

    }

    // check if long uri is a duplicate in db
    let uri_exists = validators::long_link::long_link_exists(long_uri, links_coll).await;

    // if a duplicate, return short link associated with a message
    // if not, continue
    match uri_exists {
        Some(short_uri) => {
            let msg = format!("uri submitted already exists,\nlink -> {}", short_uri);
            return msg;
        }
        None => {}
    };

    //-------------------------------------------
    // create new link if checks passed
    // doc structure

    let mut doc = Link {
        oid: None,
        created_on: DateTime::now(),
        long_uri: long_uri.to_string(),
        link_domain: "pil.ink".to_string(),
        link_id: unique_string::link_short_id(),
    };

    // changes doc short id if it is a duplicate in db
    validators::short_link::short_id_valid(&mut doc, links_coll).await;

    let insertion_status = &links_coll
        .insert_one(&doc, None).await;

    match insertion_status {
        Ok(status) => {
            println!("Link created with oid:{}", status.inserted_id.to_string());
            format!("here is your link: http://{}/{}\n", &doc.link_domain.to_string(), &doc.link_id.to_string())
        }

        Err(error) => {
            println!("Error while trying to insert a document:\n{}", error.to_string());
            format!("sad operation x_x\n")
        }
    }
}
