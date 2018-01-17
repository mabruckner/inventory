#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod schema;
use schema::items;

mod db;
use db::*;

mod classes;
use classes::*;

mod batches;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use r2d2_diesel::ConnectionManager;
use rocket::http::{Status};
use rocket::request::{self, FromRequest, Form};
use rocket::response::{Redirect, NamedFile};
use rocket::{Request, State, Outcome};
use rocket_contrib::Template;
use serde::Serialize;

use std::path::Path;

#[derive(Queryable, Debug, Serialize, Clone)]
struct Batch {
    id: i32,
    quantity: String,
    data: String,
    class: Class
}

#[derive(Queryable, Debug, Serialize, Clone)]
struct Item {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct Index {
    items: Vec<Item>
}

#[derive(Insertable)]
#[table_name="items"]
struct InsItem {
    name: String
}
fn add_form(conn: Conn, data: Form<AddForm>) -> Redirect {
    use schema::items;
    let form = data.into_inner();
    println!("ADDING {:?}", form);
    let item = InsItem{
        name: form.name
    };
    diesel::insert_into(items::table)
        .values(&item)
        .execute(&*conn)
        .expect("ERROR INSERTING VALUE");
    Redirect::to("/")
}

#[get("/")]
fn index(conn: Conn) -> Template {
    println!("HELLO WORLD");
    let context = Index {
        items: items::table.load::<Item>(&*conn).expect("UNABLE TO LOAD ITEMS").iter().cloned().collect::<Vec<Item>>()
    };
    Template::render("index", &context)
}

#[get("/delete/<id>")]
fn delete(conn: Conn, id: i32) -> Redirect {
    use schema::items;
    println!("{:?}", items::table.find(id));
    diesel::delete(items::table.find(id))
        .execute(&*conn)
        .expect("UNABLE TO DELETE");
    Redirect::to("/")
}

#[get("/static/<file>")]
fn client(file: String) -> Option<NamedFile> {
    let path = Path::new("client/target/asmjs-unknown-emscripten/debug/").join(file);
    NamedFile::open(path).ok()
}

#[derive(FromForm, Debug)]
struct AddForm {
    name: String
}
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(init_pool())
        .mount("/", routes![index,client])
        .mount("/batches", routes![batches::get_batches, delete, batches::add_batch])
        .mount("/classes", routes![classes::class_list, classes::modify_class_page, classes::delete_class, classes::add_class, classes::modify_class])
        .mount("/api/classes", routes![classes::api_get])
        .mount("/api/batches", routes![batches::api_get])
        .launch();
    println!("Hello, world!");
}
