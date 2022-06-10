#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! tesing 123"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}