
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use hero::Hero;
use rocket_contrib::{Json, Value};

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "hero 1", 
        "hero 2"
    ]))
}
#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
fn main() {
        rocket::ignite()
            .mount("/hello", routes![hello])
            .launch();
}

