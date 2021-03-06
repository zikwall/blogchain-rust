#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index])
        .launch();
}