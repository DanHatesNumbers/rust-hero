#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod routes;
mod models;

fn main() {
    rocket::ignite()
        .mount("/hero", routes![routes::create, routes::update, routes::delete])
        .mount("/heroes", routes![routes::get_all])
        .launch();
}