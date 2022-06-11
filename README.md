# rust-rocket-api
 
this is the backend for the main project here:


# https://github.com/P3t3rtm/flutter-app

# Rocket notes:

- routes
    - mounts the 'end' part of the url
    - responds & handles RESTful requests
- mounting
    - mounts the routes you defined, along with a base URL, the 'front' part of the url
    - can mount multiple base urls to same route(s)
- example routing/mounting: 
    - route: 
        - #[get("/test3/test4")]              
fn nameofroute() -> &'static str { 
    "test message 1"
}
        - #[get("/test5/test6")]              
fn nameofroute2() -> &'static str { 
    "test message 2"
}
    - mounting:
        - .mount("/test1/test2", routes![nameofroute,nameofroute2])
    - resulting urls:
        - example.com/test1/test2/test3/test4:
            - OK (200): test message 1
        - example.com/test1/test2/test5/test6:
            - OK (200): test message 2