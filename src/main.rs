use std::env;
use serde::Deserialize;
#[derive(Deserialize)]
struct ExchangeRate {
    date: String,
    base: String,
    quote: String,
    rate: f64,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let from_currency: &String = &args[1];
    let to_currency: &String = &args[2];
    let baseurl: String = String::from("https://api.frankfurter.dev/v2");
    let url: String = format!("{}/rates?base={}&quotes={}", baseurl, &from_currency, &to_currency);
    let response = ureq::get(&url)
        .call()
        .expect("Request error");

    // println!("Hello, world! args={:?}", args);
    let data: Vec<ExchangeRate> = response.into_body().read_json().expect("Read error");
    if let Some(first_rate) = data.first() {
        println!("1 {} = {} {} (дата {})", first_rate.base, first_rate.rate, first_rate.quote, first_rate.date);
    }
}

