use db::*;
use diesel::prelude::*;
use diesel::{self, BelongingToDsl};
use schema::{classes,batches};
use rocket_contrib::Template;
use schema;
use rocket::request::{self, FromRequest, Form};
use rocket::response::Redirect;

use classes::{Class, Batch};

#[derive(Queryable, Debug, Clone, Serialize)]
pub struct BatchPair {
    batch: Batch,
    class: Class
}

#[derive(Serialize)]
pub struct BatchesPage {
    batches: Vec<BatchPair>,
    classes: Vec<Class>
}

#[derive(FromForm, Insertable, AsChangeset, Debug)]
#[table_name="batches"]
pub struct BatchForm {
    class: i32, 
    quantity: String,
    data: String
}

#[post("/add", data="<data>")]
pub fn add_batch(conn: Conn, data: Form<BatchForm>) -> Redirect {
    diesel::insert_into(batches::table)
        .values(&data.into_inner())
        .execute(&*conn)
        .expect("UNABLE TO ADD");
    Redirect::to("/")
}

#[get("/")]
pub fn get_batches(conn: Conn) -> Template {
    let stuff = batches::table.inner_join(classes::table)
        .load::<BatchPair>(&*conn).expect("UNABLE TO LOAD BATCHES")
        .iter().cloned().collect::<Vec<BatchPair>>();
    let classlist = classes::table.load::<Class>(&*conn).expect("UNABLE TO LOAD CLASSES")
        .iter().cloned().collect::<Vec<Class>>();
    println!("BATCHES:");
    for x in &stuff {
        println!("{:?}", x);
    }
    let context = BatchesPage {
        batches: stuff,
        classes: classlist
    };
    Template::render("batches", &context)
}

