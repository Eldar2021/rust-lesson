use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CryptoPrice {
    pub usd: f64,
}

#[derive(Debug, Deserialize)]
pub struct CryptoPrices {
    pub bitcoin: CryptoPrice,
    pub ethereum: CryptoPrice,
    pub ripple: CryptoPrice,
}
