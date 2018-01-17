use db::*;
use diesel::prelude::*;
use diesel;
use schema::{classes, batches};

use rocket::request::{self, FromRequest, Form};
use rocket::response::Redirect;
use rocket_contrib::Json;

use rocket_contrib::Template;
#[derive(Identifiable, Queryable, Debug, Serialize, Clone)]
#[table_name = "classes"]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub schema: String
}

#[derive(Queryable, Associations, Debug, Serialize, Clone)]
#[belongs_to(Class)]
#[table_name = "batches"]
pub struct Batch {
    pub id: i32,
    pub class: i32,
    pub quantity: String,
    pub data: String
}

#[derive(Serialize)]
struct ClassList {
    classes: Vec<Class>,
    units: Vec<String>,
    default: ClassForm
}

#[derive(Serialize)]
struct ClassMod {
    class: Class,
    units: Vec<String>
}

fn get_units() -> Vec<String> {
    vec!["Count".into(), "Mass".into(), "Volume".into()]
}

#[get("/")]
pub fn api_get(conn: Conn) -> Json<Vec<Class>> {
    Json(classes::table.load::<Class>(&*conn).expect("UNABLE TO COMPLETE REQUEST").iter().cloned().collect::<Vec<Class>>())
}

#[get("/")]
pub fn class_list(conn: Conn) -> Template {
    use schema::classes;
    let context = ClassList {
        classes: classes::table.load::<Class>(&*conn).expect("UNABLE TO LOAD CLASSES").iter().cloned().collect::<Vec<Class>>(),
        units: get_units(),
        default: ClassForm {
            name: "".into(),
            unit: "Count".into(),
            schema: "".into()
        }
    };
    Template::render("classes", &context)
}

#[get("/modify/<id>")]
pub fn modify_class_page(conn: Conn, id: i32) -> Template {
    let context = ClassMod {
        class: classes::table.find(id)
               .load::<Class>(&*conn)
               .expect("CLASS DOES NOT EXIST")[0].clone(),
        units: get_units()
    };
    Template::render("modify_class", &context)
}

#[derive(FromForm, Debug, Insertable, AsChangeset, Serialize)]
#[table_name="classes"]
pub struct ClassForm {
    name: String,
    unit: String,
    schema: String
}

#[get("/delete/<id>")]
pub fn delete_class(conn: Conn, id: i32) -> Redirect {
    diesel::delete(classes::table.find(id))
        .execute(&*conn)
        .expect("UNABLE TO DELETE");
    Redirect::to("/classes")
}

#[post("/modify/<id>", data="<data>")]
pub fn modify_class(conn: Conn, data: Form<ClassForm>, id: i32) -> Redirect {
    diesel::update(classes::table.find(id))
        .set(&data.into_inner())
        .execute(&*conn)
        .expect("UNABLE TO UPDATE");
    Redirect::to("..")
}

#[post("/add", data="<data>")]
pub fn add_class(conn: Conn, data: Form<ClassForm>) -> Redirect {
    diesel::insert_into(classes::table)
        .values(&data.into_inner())
        .execute(&*conn)
        .expect("UNABLE TO ADD");
    Redirect::to("/classes")
}
