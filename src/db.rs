use dotenv;
use diesel::prelude::*;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;
use rocket::http::{Status};
use rocket::request::{self, FromRequest, Form};
use rocket::{Request, State, Outcome};

use schema::items;

use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let manager = ConnectionManager::<SqliteConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL is not set"));
    r2d2::Pool::new(manager).expect("unable to create db Pool")
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATEBASE_URL is not set");
    SqliteConnection::establish(&database_url).expect(&format!("error connecting to database {:?}", database_url))
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

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

