use indicatif::ProgressBar;

use crate::models::CryptoPrices;

pub async fn fetch_crypto_prices(pb: &ProgressBar) -> Result<CryptoPrices, reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,ripple&vs_currencies=usd";

    let response = reqwest::get(url).await?;

    for i in 0..100 {
        pb.set_position(i);
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    }

    let prices: CryptoPrices = response.json().await?;

    Ok(prices)
}
