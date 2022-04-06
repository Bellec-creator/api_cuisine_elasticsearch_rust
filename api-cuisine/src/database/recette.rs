use serde::Deserialize;
use serde::Serialize;

use rocket_okapi::JsonSchema;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub titre: String,
    pub ingredient: Vec<Ingredient>,
    pub etape: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub name: String,
    #[serde(rename = "quantité")]
    pub quantit: String,
}
