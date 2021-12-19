use rand::seq::IteratorRandom;
use rocket_contrib::json::Json;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Serialize)]
pub struct Quote {
    pub quote: String,
    pub subject: String,
}

#[get("/")]
pub fn random() -> Json<Quote> {
    let f = File::open("../../src/quotes.txt").expect("Could not read file");
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    let quote = lines
        .choose(&mut rand::thread_rng())
        .expect("Error choosing a line");
    let split_quote = quote.split("|");
    let quote_vec: Vec<&str> = split_quote.collect();
    Json(Quote {
        quote: quote_vec[0].trim().to_string(),
        subject: quote_vec[1].trim().to_string(),
    })
}
