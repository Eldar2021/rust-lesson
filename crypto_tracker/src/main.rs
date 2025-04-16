mod api;
mod models;
use api::fetch_crypto_prices;

#[tokio::main]
async fn main() {
    println!("Fetching cryptocurrency prices...");

    match fetch_crypto_prices().await {
        Ok(prices) => {
            println!("Bitcoin: ${}", prices.bitcoin.usd);
            println!("Ethereum: ${}", prices.ethereum.usd);
            println!("Ripple: ${}", prices.ripple.usd);
        }
        Err(e) => {
            eprintln!("Error fetching prices: {}", e);
            return;
        }
    }
}
