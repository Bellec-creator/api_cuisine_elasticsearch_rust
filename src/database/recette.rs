use serde::Deserialize;
use serde::Serialize;

use rocket_okapi::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub titre: String,
    pub recette_type: RecetteType,
    pub ingredient: Vec<Ingredient>,
    pub etape: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub name: String,
    #[serde(rename = "quantité")]
    pub quantit: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum RecetteType {
    #[serde(rename = "Petit déjeuner", alias = "petit'dej")]
    PetitDejeuner,
    Aperitif,
    Salade,
    Entree,
    Sauce,
    Plat,
    Dessert,
    Biscuits,
    Boissons,
}
