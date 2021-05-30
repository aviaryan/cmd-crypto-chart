mod utils;

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PriceData {
    pub prices: Vec<(f64, f64)>,
}

pub async fn get_prices() -> Result<PriceData, Error> {
    let unix_time = utils::get_unix_time();
    // println!("{}", unix_time);

    // set user agent
    let client = reqwest::Client::builder().build()?;

    // from https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
    let request_url = format!(
        "https://api.coingecko.com/api/v3/coins/{coin}/market_chart/range?vs_currency=usd&from={from}&to={to}",
        coin = "bitcoin",
        from = unix_time / 1000 - (24 * 60 * 60),
        // ^ from current time - 8hr to {NOW}
        to = unix_time / 1000
    );
    // println!("{}", request_url);
    let response = client.get(&request_url).send().await?;

    // println!("Status: {}", response.status());
    // println!("Status: {:#?}", response.text().await?);

    let prices: PriceData = response.json().await?;
    // println!("{:?}", prices);
    // println!("{}", prices.prices.len());
    Ok(prices)
}
