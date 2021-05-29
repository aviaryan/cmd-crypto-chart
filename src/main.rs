// fn main() {
//     println!("Hello, world!");
// }

use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // set user agent
    let client = reqwest::Client::builder()
        .user_agent("aviaryan")
        .build()?;

    // from https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let response = client.get(&request_url).send().await?;

    // println!("Status: {}", response.status());
    // println!("Status: {:#?}", response.text().await?);

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
