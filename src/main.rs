#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod controller;
use controller::todo::DbConnection;

//use rocket::{self, get,routes};
mod routes;
mod models;
mod schema;

#[get("/hello")]
fn hello()->&'static str{
    "hello world"
}

fn main() {
    rocket::ignite()
        .attach(DbConnection::fairing())
        .mount("/hello", routes![hello])
        .mount("/todo", routes![routes::get_todo, routes::create_todos, routes::check])
        .launch();
}
