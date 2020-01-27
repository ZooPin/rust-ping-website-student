/*
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod business;
mod database;

use rocket::http::Method;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error, Cors, CorsOptions };

fn cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080"
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods:vec![
            Method::Get,
            Method::Post
        ].into_iter().map(From::from).collect(),
        allowed_headers:AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin"
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![crate::business::ping_service::ping_endpoint])
        .attach(cors())
}

fn main() -> Result<(), Error> {
    let _connection = database::lib::establish_connection();

    rocket().launch();

    Ok(())
}
