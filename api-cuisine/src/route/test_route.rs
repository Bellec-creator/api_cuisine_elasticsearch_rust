use elasticsearch::{Elasticsearch, SearchParts};
use elastiql::search::{
    query::{BooleanQuery, CompoundQuery, Query, TermQuery},
    HighlightOptions, Request, Response, OkResponse,
};
use rocket::response::status::BadRequest;
use rocket::State;
use rocket::serde::json::Json;
use rocket::tokio::time::{sleep, Duration};
use serde_json::{json, Value};
use crate::{App};
use crate::database::recette::Root;


#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("waited for {} seconds", seconds)
}

#[get("/world")]
pub async fn world(app : &State<App>) -> Result<Json<Vec<Root>>, BadRequest<String>> {
    let client = &app.elasticsearch;
  
  
    // let query = TermQuery::new("id", "test_user_id");
    // // which, when passed to `Request.builder().query()`, is short for:
    // let query = CompoundQuery::builder()
    //     .boolean(
    //         BooleanQuery::builder()
    //             .filter(vec![Query::builder().term(query).build()])
    //             .build()
    //     )
    //     .build();
  
  
    let matching =  TermQuery::new("titre.keyword","Crêpes : la meilleure recette rapide");
    let q = Query::builder().term(matching).build();
    let query = CompoundQuery::builder()
        .boolean(
            Some(BooleanQuery::builder()
                .filter(vec![q])
                .build())
        )
        .build();

    // let query = Query::builder()
    // .term(Some(matching)).build();
    let request = Request::builder()
    .query(query).build();

    let search_response = client
        .search(SearchParts::Index(&["recettes"]))
        .body(request)
        .send()
        .await;

     // let search_response = client
    //     .search(SearchParts::Index(&["recettes"]))
    //     .body(json!({
    //         "query": {
    //             "term": {
    //                 "titre": "Crêpes : la meilleure recette rapide"
    //             }
    //         }
    //     }))
    //     .send()
    //     .await;


    let response_body = search_response.unwrap().json::<OkResponse<Root>>().await.unwrap();
    let hits = response_body.hits.hits.into_iter().map(|h|h.source).collect();
    
    Ok(Json(hits))

}
#[get("/name")]
pub async fn name() -> &'static str {
    "Hello, name!"
}

