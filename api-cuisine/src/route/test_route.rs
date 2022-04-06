use elasticsearch::{Elasticsearch, Error, http, IndexParts, SearchParts};
use elastiql::search::{
    query::{BooleanQuery, CompoundQuery, Query, TermQuery},
    HighlightOptions, Request, Response, OkResponse,
};
use elastiql::search::query::MatchQuery;
use rocket::http::hyper::StatusCode;
use rocket::http::RawStr;
use rocket::response::status::BadRequest;
use rocket::State;
use rocket::serde::json::Json;
use rocket::tokio::time::{sleep, Duration};
use rocket_okapi::{openapi, settings::OpenApiSettings, openapi_get_routes};
use serde_json::{json, Value};
use crate::{App, client};
use crate::database::recette::Root;

pub fn load_route(
    loader: rocket::Rocket<rocket::Build>,
    settings: &OpenApiSettings
) -> rocket::Rocket<rocket::Build> {
    loader.mount(
        "/api",
        openapi_get_routes![get_recette, post_recette]
    )
}

#[openapi]
#[get("/recette?<ingrediant>")]
pub async fn get_recette(app : &State<App>, ingrediant: &str) -> Result<Json<Vec<Root>>, BadRequest<String>> {
    let client = &app.elasticsearch;

    let search_response = client
         .search(SearchParts::Index(&["recettes"]))
         .body(json!({
             "query": {
                 "match": {
                     "ingredient.name": ingrediant
                 }
             }
         }))
         .send()
         .await;
    let response_body = search_response.unwrap().json::<OkResponse<Root>>().await.unwrap();
    let hits = response_body.hits.hits.into_iter().map(|h|h.source).collect();

    Ok(Json(hits))

}

#[openapi]
#[post("/recette", data = "<recette>")]
pub async fn post_recette(app : &State<App>, recette: Json<Root>)-> Result<String,BadRequest<String>> {
    let document = recette.into_inner();
    let client = &app.elasticsearch;

    let post_document = client
        .index(IndexParts::Index("recettes"))
        .body(json!(document))
        .send()
        .await;

    match post_document {
        Ok(t) => {
            rocket::serde::__private::Result::Ok(t.status_code().to_string())
        },
        Err(e) => {
            rocket::serde::__private::Result::Err(BadRequest(Some(e.to_string())))
        }
    }
    //let successful = post_document.ok().flatten().status_code().as_str();

    //Ok(successful)

}


