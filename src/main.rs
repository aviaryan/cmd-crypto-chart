mod api;
mod chart;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let prices = api::get_prices().await?;
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
