/* #region  japanese imports */
#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};
#[derive(Database)]
#[database("db_rocket")]
struct Data(sqlx::MySqlPool);

/* #endregion */






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
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::Forbidden, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)),
        }
    }
}






/* #region  routes */

#[get("/sensitive")]
fn sensitive(key: ApiKey<'_>) -> &'static str {
    "Sensitive data."
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world! tesing 123"
}

#[get("/<id>")]
async fn read(mut db: Connection<Data>, id: i64) -> Option<String> {
    sqlx::query("SELECT content FROM logs WHERE id = ?")
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
