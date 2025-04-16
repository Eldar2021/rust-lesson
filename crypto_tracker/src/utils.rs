use std::fmt::Write;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use prettytable::{color, row, Attr, Cell, Row, Table};

use crate::models::CryptoPrices;

pub fn init_progress_bar(pb: &ProgressBar) {
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
       .unwrap()
       .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-"));
}

pub fn display_prices(prices: CryptoPrices) {
    let mut table = Table::new();

    table.add_row(row!["Cryptocurrency", "Price (USD)"]);

    table.add_row(Row::new(vec![
        Cell::new("Bitcoin").with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new(&format!("${:.2}", prices.bitcoin.usd))
            .with_style(Attr::ForegroundColor(color::YELLOW)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Ethereum").with_style(Attr::ForegroundColor(color::CYAN)),
        Cell::new(&format!("${:.2}", prices.ethereum.usd))
            .with_style(Attr::ForegroundColor(color::YELLOW)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Ripple").with_style(Attr::ForegroundColor(color::BLUE)),
        Cell::new(&format!("${:.2}", prices.ripple.usd))
            .with_style(Attr::ForegroundColor(color::YELLOW)),
    ]));

    table.printstd();
}
