mod test;
mod database;
mod route;
use std::fmt::format;
use std::future::Future;
use elasticsearch::Elasticsearch;
use rocket::{Build, Rocket, State};
use rocket::tokio::runtime::Runtime;
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig, Theme, UiConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use crate::database::recette::Root;
use crate::route::test_route::{get_recette, post_recette};
use crate::database::connection::client;


#[macro_use] extern crate rocket;

fn load_route(loader: Rocket<Build>) -> Rocket<Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    let loader = route::test_route::load_route(loader, &settings);

    loader
}


#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let loader = rocket::build()
        .manage(App::new());

    let loader = load_route(loader);

    let urls = vec![
        UrlObject::new("recette", "/api/openapi.json")
    ];
    let loader = loader.mount(
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
            })
        );

    Ok(loader.launch().await?)
}

pub struct App {
    elasticsearch: Elasticsearch,
    async_rt: Runtime,
}

pub type AppState<> = State<App>;

impl App {
    fn new() -> Self {
        let rt = Runtime::new().expect("Tokio runtime can be created");
        let elasticsearch = client().unwrap();

        Self { elasticsearch, async_rt: rt }
    }

    /// Run given future in async runtime and block current thread until it resolves.
    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.async_rt.handle().block_on(future)
    }
}

pub(crate) trait WithElastic {
    /// Get reference to stateful Elasticsearch client.
    fn elasticsearch(&self) -> &Elasticsearch;
}

impl WithElastic for AppState {
    fn elasticsearch(&self) -> &Elasticsearch {
        &self.elasticsearch
    }
}


