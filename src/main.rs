#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod schema;
use schema::items;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use r2d2_diesel::ConnectionManager;
use rocket::http::{Status};
use rocket::request::{self, FromRequest, Form};
use rocket::response::Redirect;
use rocket::{Request, State, Outcome};
use rocket_contrib::Template;
use serde::Serialize;

use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn init_pool() -> Pool {
    dotenv().ok();
    let manager = ConnectionManager::<SqliteConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL is not set"));
    r2d2::Pool::new(manager).expect("unable to create db Pool")
}

#[derive(Queryable, Debug, Serialize, Clone)]
struct Item {
    id: i32,
    name: String
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

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATEBASE_URL is not set");
    SqliteConnection::establish(&database_url).expect(&format!("error connecting to database {:?}", database_url))
}

struct Conn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Conn {
    type Target = SqliteConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

#[derive(FromForm, Debug)]
struct AddForm {
    name: String
}

#[post("/add", data="<data>")]
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

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(init_pool())
        .mount("/", routes![index, add_form, delete])
        .launch();
    println!("Hello, world!");
}
