use crate::database::recette::Root;
use crate::App;
use elasticsearch::{Error, IndexParts, SearchParts};
use elastiql::search::OkResponse;
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, State};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::{openapi, openapi_get_routes, settings::OpenApiSettings, OpenApiError};
use serde_json::json;

pub enum ApiError {
    Elastic(elasticsearch::Error),
}

impl From<elasticsearch::Error> for ApiError {
    fn from(e: Error) -> Self {
        Self::Elastic(e)
    }
}

//loader pour les routes et ajout des paramettre pour le swagger
pub fn load_route(
    loader: rocket::Rocket<rocket::Build>,
    _settings: &OpenApiSettings,
) -> rocket::Rocket<rocket::Build> {
    loader.mount("/api", openapi_get_routes![get_recette, post_recette]) //montage des route sur /api
}

#[openapi] // nécessaires pour faire apparaître la route dans le swagger
#[get("/recette?<ingrediant>")] // definition du endpoint
pub async fn get_recette(app: &State<App>, ingrediant: &str) -> Result<Json<Vec<Root>>, ApiError> {
    let client = &app.elasticsearch; // recuperation du state
    let search_response = client // creation de la requête
        .search(SearchParts::Index(&["recettes"]))
        .body(json!({
            "query": {
                "match": {
                    "ingredient.name": {
                        "query" : ingrediant,
                        "fuzziness" : "AUTO"
                    }
                }
            }
        }))
        .send()
        .await;
    let response_body = search_response?.json::<OkResponse<Root>>().await?; // recupération de la recette
    let hits = response_body
        .hits
        .hits
        .into_iter()
        .map(|h| h.source)
        .collect(); // recuperation de la partie "_source" du doc ElasticSearch

    Ok(Json(hits))
}

#[openapi]
#[post("/recette", data = "<recette>")]
pub async fn post_recette(app: &State<App>, recette: Json<Root>) -> Result<String, Custom<String>> {
    let document = recette.into_inner();
    let client = &app.elasticsearch;

    let post_document = client
        .index(IndexParts::Index("recettes"))
        .body(json!(document))
        .send()
        .await;

    // gestion des erreurs
    match post_document {
        Ok(_) => Ok("Upload document".to_string()),
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string())),
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        let mut builder = response::Response::build();
        builder.header(ContentType::Plain);
        builder.status(Status::InternalServerError);
        match self {
            ApiError::Elastic(e) => {
                let s = e.to_string();
                builder.sized_body(s.len(), std::io::Cursor::new(s));
            }
        }
        builder.ok()
    }
}

impl OpenApiResponderInner for ApiError {
    fn responses(_gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        let responses = Responses::default();
        // let schema = gen.json_schema::<String>();
        // rocket_okapi::util::add_schema_response(&mut responses, 200, "text/plain", schema)?;
        Ok(responses)
    }
}
