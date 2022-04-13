mod database;
mod route;
use crate::database::connection::client;
use elasticsearch::Elasticsearch;
use rocket::{Build, Rocket, State};
use rocket_okapi::rapidoc::{
    make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig, Theme, UiConfig,
};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[macro_use]
extern crate rocket;

// creation du loader pour le swagger
fn load_route(loader: Rocket<Build>) -> Rocket<Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    let loader = route::test_route::load_route(loader, &settings);

    loader
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let loader = rocket::build().manage(App::new()?);

    let loader = load_route(loader);

    // documentation swagger

    let urls = vec![UrlObject::new("recette", "/openapi.json")];
    let loader = loader
        .mount(
            "/doc/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                urls: urls.clone(),
                ..Default::default()
            }),
        )
        .mount(
            "/doc/rapidoc",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: urls,
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    Ok(loader.launch().await?)
}
//
pub struct App {
    elasticsearch: Elasticsearch,
}

pub type AppState = State<App>;

impl App {
    fn new() -> anyhow::Result<Self> {
        let elasticsearch = client()?;
        Ok(Self { elasticsearch })
    }
    pub fn elasticsearch(&self) -> &Elasticsearch {
        &self.elasticsearch
    }
}
