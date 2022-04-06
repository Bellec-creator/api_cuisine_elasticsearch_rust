mod test;
mod database;
mod route;
use std::fmt::format;
use std::future::Future;
use elasticsearch::Elasticsearch;
use rocket::{Build, Rocket, State};
use rocket::tokio::runtime::Runtime;
use crate::database::recette::Root;
use crate::route::test_route::{delay, world, name};
use crate::database::connection::client;

#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().manage(App::new()).mount("/hello", routes![world, name, delay])
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


