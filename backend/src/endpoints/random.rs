use rocket_contrib::json::Json;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use rand::seq::IteratorRandom;

#[derive(Serialize)]
pub struct Quote {
    pub quote: String,
    pub source: String,
}

#[get("/")]
pub fn random() -> Json<Quote> {
    let f = File::open("quotes.txt")
    .expect("Could not read file");
    let f  = BufReader::new(f);
    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    let quote = lines.choose(&mut rand::thread_rng()).expect("Error");
    let split_quote = quote.split("|");
    let quote_vec: Vec<&str> = split_quote.collect();
    Json(Quote {
        quote: quote_vec[0]
            .trim()    
            .to_string(),
        source: quote_vec[1]
            .trim()
            .to_string(),
    })
}