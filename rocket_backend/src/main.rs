use rocket_db_pools::mongodb::Collection;
use crate::libs::config_structs::DbConfig;
use crate::models::links::Link;

#[macro_use]
extern crate rocket;

mod routes;
mod libs;
mod models;

#[launch]
async fn rocket() -> _ {

    // merged configs
    let figment = libs::config_parse::super_config();

    // deserialized config for links db
    let db_link_config: DbConfig = figment.clone().select("database_links").extract()
        .expect("Failed to read database_links section from config.");

    // connection to mongodb collection
    let links_coll: Collection<Link> = libs::mongodb::mongodb_session(&db_link_config.conn_uri).await
        .database(&db_link_config.db_name)
        .collection(&db_link_config.db_coll);

    rocket::custom(figment.clone())

        .mount("/", routes![
            routes::redirects::redirect_to_long,
            routes::root_route::welcome,
            routes::root_route::not_found,
        ])

        .mount("/link", routes![
            routes::link_route::create_link_page,
            routes::link_route::create_link_request,
        ])

        .manage(db_link_config)
        .manage(links_coll)
        .attach(rocket_dyn_templates::Template::fairing())
}
