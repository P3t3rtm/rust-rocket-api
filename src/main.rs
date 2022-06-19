/* #region  japanese imports */
#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
#[derive(Database)]
#[database("db_rocket")]
struct Data(sqlx::MySqlPool);








const XAPIKEY: &str = "xapikey";
/* #endregion */



/* #region  functions */


struct ApiKey<'r>(&'r str);
#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == XAPIKEY
        }

        match req.headers().get_one("apikey") {
            None => Outcome::Failure((Status::Forbidden, ApiKeyError::Missing)), //headers contains no "apikey"
            Some(keyd) if is_valid(keyd) => Outcome::Success(ApiKey(keyd)), //headers contains "apikey" and it is valid
            Some(_) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)), //headers contains "apikey" and it is invalid
        }
    }
}



/* #endregion */

/* #region  routes */

// #[get("/production")]

#[get("/sensitive")]
fn sensitive(_xkey:ApiKey) -> &'static str {
    "Sensitive data."
}

#[get("/")]
fn index(_a:ApiKey) -> &'static str {
    "Hello, world! tesing 123"
}

#[get("/<id>")]
async fn read(mut db: Connection<Data>, id: i64) -> Option<String> {
    sqlx::query("SELECT firstname FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
} //something

use rocket::tokio::time::{sleep, Duration};
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

/* #endregion */

/* #region  main program */
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Data::init())
        //mounts are the front part of the urls
        .mount("/", routes![index, delay])
        .mount("/2/testing", routes![read, index])
        .mount("/test2", routes![read])
        .mount("/sensitive", routes![sensitive])
        .mount("/public", FileServer::from("static/"))
}
/* #endregion */
