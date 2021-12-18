use rocket_contrib::json::Json;
use rand::Rng;
use crate::get_quote;
use crate::get_quote::Quote;

#[get("/")]
pub fn random() -> Json<Quote> {
    let num = rand::thread_rng().gen_range(0..13);
    let quote = get_quote::ret_quote(num);
    Json(Quote {
        quote: quote.quote,
        source: quote.source
    })
}