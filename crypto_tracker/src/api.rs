use crate::models::CryptoPrices;

pub async fn fetch_crypto_prices() -> Result<CryptoPrices, reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,ripple&vs_currencies=usd";

    let response = reqwest::get(url).await?;
    let prices: CryptoPrices = response.json().await?;

    Ok(prices)
}
