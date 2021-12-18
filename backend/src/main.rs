#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_cors::{AllowedHeaders};
use rocket::http::Method;

mod endpoints {
    pub mod random;
}
pub mod get_quote;

fn main() {
    let cors = rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
      }
      .to_cors()
      .expect("Error creating cors");

    rocket::ignite()
    .mount("/random", routes![endpoints::random::random])
    .attach(cors)
    .launch();
}


