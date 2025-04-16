mod api;
mod models;
mod utils;

use api::fetch_crypto_prices;
use indicatif::ProgressBar;
use utils::{display_prices, init_progress_bar};

#[tokio::main]
async fn main() {
    println!("Fetching cryptocurrency prices...");

    let pb = ProgressBar::new(100);

    init_progress_bar(&pb);

    match fetch_crypto_prices(&pb).await {
        Ok(prices) => {
            pb.finish_with_message("Prices fetched successfully!");
            display_prices(prices);
        }
        Err(e) => {
            pb.finish_with_message("Error fetching prices.");
            eprintln!("Error fetching prices: {}", e);
            return;
        }
    }
}
