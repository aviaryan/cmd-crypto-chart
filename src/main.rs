mod api;
mod chart;

use reqwest::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // check args for coin name
    let args: Vec<String> = env::args().collect();
    let mut coin = "bitcoin";
    if args.len() > 1 {
        coin = &args[1];
    }
    // println!("{}", coin);

    let prices = api::get_prices(coin.to_string()).await?;
    // println!("{:?}", prices);
    println!("{}", prices.prices.len());

    // https://stackoverflow.com/a/53368681/2295672
    // handle error part of Result
    match chart::draw(prices.prices) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    // chart::calc(prices.prices);

    Ok(())
}
