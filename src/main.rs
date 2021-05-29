// fn main() {
//     println!("Hello, world!");
// }

use reqwest::Error;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug)]
struct PriceData {
    prices: Vec<(i64, f32)>
}

// credits https://stackoverflow.com/a/44378174/2295672
fn get_unix_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    // println!("{:?}", since_the_epoch);
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let unix_time = get_unix_time();
    // println!("{}", unix_time);
    // Ok(())

    // set user agent
    let client = reqwest::Client::builder().build()?;

    // from https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
    let request_url = format!(
        "https://api.coingecko.com/api/v3/coins/{coin}/market_chart/range?vs_currency=usd&from={from}&to={to}",
        coin = "bitcoin",
        from = unix_time / 1000 - (8 * 60 * 60),
        // ^ from current time - 8hr to {NOW}
        to = unix_time / 1000
    );
    println!("{}", request_url);
    let response = client.get(&request_url).send().await?;

    // println!("Status: {}", response.status());
    // println!("Status: {:#?}", response.text().await?);

    let prices: PriceData = response.json().await?;
    println!("{:?}", prices);
    Ok(())
}
