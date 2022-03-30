use elasticsearch::{Elasticsearch, SearchParts};
use rocket::State;
use rocket::tokio::time::{sleep, Duration};
use serde_json::{json, Value};
use crate::database::connection::client;
use crate::{App, AppState, WithElastic};


#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("waited for {} seconds", seconds)
}

#[get("/world")]
pub async fn world(app : &State<App>) -> String {
    let client = &app.elasticsearch;
    let search_response = client
        .search(SearchParts::Index(&["recettes"]))
        .from(0)
        .body(json!({
            "query": {
                "match": {
                    "message": "Elasticsearch rust"
                }
            }
        }))
        .send()
        .await;

    let response_body = search_response.unwrap().json::<Value>().await.unwrap();
    let took = response_body["took"].as_i64().unwrap();
    //return format!("{}",response_body["hits"]["hits"]);
    let hits = response_body["hits"]["hits"].as_str();
    match hits {
        None => format!("test"),
        Some(hits) =>  format!("{}", hits),
    }

}
#[get("/name")]
pub async fn name() -> &'static str {
    "Hello, name!"
}