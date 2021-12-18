#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod endpoints {
    pub mod random;
}
pub mod get_quote;

fn main() {
    rocket::ignite().mount("/random", routes![endpoints::random::random]).launch();
}


