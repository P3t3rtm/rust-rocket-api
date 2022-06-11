#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! tesing 123"
}

#[get("/3/test3")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world! gay"
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
        .mount("/", routes![index,delay])
        .mount("/2/testing", routes![world,index])
        .mount("/testing123", routes![world])
}

