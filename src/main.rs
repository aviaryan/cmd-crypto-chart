// fn main() {
//     println!("Hello, world!");
// }

// mod utils;
mod api;

use reqwest::Error;
// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// struct PriceData {
//     prices: Vec<(u64, f32)>,
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    let prices = api::get_prices().await?;
    // let unix_time = utils::get_unix_time();
    // // println!("{}", unix_time);
    // // Ok(())

    // // set user agent
    // let client = reqwest::Client::builder().build()?;

    // // from https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
    // let request_url = format!(
    //     "https://api.coingecko.com/api/v3/coins/{coin}/market_chart/range?vs_currency=usd&from={from}&to={to}",
    //     coin = "bitcoin",
    //     from = unix_time / 1000 - (24 * 60 * 60),
    //     // ^ from current time - 8hr to {NOW}
    //     to = unix_time / 1000
    // );
    // println!("{}", request_url);
    // let response = client.get(&request_url).send().await?;

    // // println!("Status: {}", response.status());
    // // println!("Status: {:#?}", response.text().await?);

    // let prices: PriceData = response.json().await?;
    println!("{:?}", prices);
    println!("{}", prices.prices.len());

    Ok(())
}
