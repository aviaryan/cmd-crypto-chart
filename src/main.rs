mod api;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let prices = api::get_prices().await?;
    println!("{:?}", prices);
    println!("{}", prices.prices.len());

    Ok(())
}
