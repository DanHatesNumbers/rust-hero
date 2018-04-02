use rocket_contrib::{Json, Value};

use models::Hero;

#[post("/", data = "<hero>")]
pub fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
pub fn get_all() -> Json<Value> {
    Json(json!([
        "hero 1",
        "hero 2"
    ]))
}

#[put("/<id>", data = "<hero>")]
pub fn update(id: u32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
pub fn delete(id: u32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}