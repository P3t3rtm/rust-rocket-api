#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! tesing 123"
}

#[get("/3/test3")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world! gay"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/2/testing", routes![world,index])
        .mount("/testing123", routes![world])
}

