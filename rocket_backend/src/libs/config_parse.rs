use rocket::{ figment::Figment };
use rocket::figment::providers::{Toml, Format};

pub fn super_config() -> Figment {
    let config:Figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Architecture.toml").nested());
    config
}
