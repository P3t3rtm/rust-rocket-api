#[macro_use] extern crate rocket;

use rocket_db_pools::{
    Database,
    Connection
};

use rocket_db_pools::sqlx::{
    self,
    Row
};

#[derive(Database)]
#[database("db_rocket")]
struct Data(sqlx::MySqlPool);


#[get("/")]
fn index() -> &'static str {
    "Hello, world! tesing 123"
}

#[get("/<id>")]
async fn read(mut db: Connection<Data>, id: i64) -> Option<String> {
   sqlx::query("SELECT content FROM logs WHERE id = ?").bind(id)
       .fetch_one(&mut *db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .ok()
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Data::init())
        .mount("/", routes![index,delay])
        .mount("/2/testing", routes![read,index])
        .mount("/test2", routes![read])
}

