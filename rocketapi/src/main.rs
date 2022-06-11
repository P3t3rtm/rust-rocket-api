#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! tesing 123"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world! gay"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/testing", routes![world,index])
        .mount("/testing123", routes![world])
}