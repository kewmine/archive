use {
    crate::libs,
    rocket_dyn_templates::{context, Template}};

use libs::config_structs::{
    TemplateConfig
};

#[get("/")]
pub fn welcome() -> Template {

    // Merged Configs; Architecture supersedes Rocket.
    let figment = libs::config_parse::super_config();

    let template_config: TemplateConfig = figment.clone().select("template-content").extract()
        .expect("Failed to read [template-content] from config");

    Template::render("pages/welcome", context! {name: template_config.author})
}

#[get("/404")]
pub fn not_found() -> String {
    format!("\n404: not found\n")
}
