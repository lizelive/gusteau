#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Def {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Thing {
    #[serde(flatten)]
    pub def: Def,
}

pub struct MarketValue {
    pub thing: String,
    pub price: f64,
}

impl Thing {
    pub fn title(&self) -> &String {
        self.def.title.as_ref().unwrap_or(&self.def.name)
    }
    pub fn name(&self) -> &String {
        &self.def.name
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Filter {
    Thing(Vec<String>),
    Category(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ingredient {
    pub filter: Filter,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub thing: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Bench {
    #[serde(flatten)]
    pub def: Def,
    pub recipes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Labor {
    pub amount: f64,
    pub skill_stat: String,
    pub speed_stat: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    #[serde(flatten)]
    pub def: Def,
    pub labor: Labor,
    pub ingredient: Vec<Ingredient>,
    pub products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    pub things: Vec<Thing>,
    pub benches: Vec<Bench>,
    pub recipes: Vec<Recipe>,
}
