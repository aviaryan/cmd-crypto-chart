use serde::Deserialize;

pub type PriceInfo = Vec<(f64, f64)>;

#[derive(Deserialize, Debug)]
pub struct PriceData {
    pub prices: PriceInfo,
}
