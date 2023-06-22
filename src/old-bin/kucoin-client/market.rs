use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use clap::Parser;
use clap::Subcommand;
use kucoin_api::client::Kucoin;
use kucoin_api::client::KucoinEnv;
use kucoin_api::model::market::OrderBookType;
use kucoin_api::model::parameters::klines::Klines;
use kucoin_api::model::request::market::CandleRequest;

#[derive(Subcommand, Debug)]
pub enum Market {
    /// Get a list of all symbols
    Symbols(Symbols),
    /// Get ticker for a symbol
    Ticker(Ticker),
    /// Get all tickers for all symbols
    AllTickers,
    /// Get daily stats for a symbol
    DailyStats(DailyStats),
    /// Get a list of all markets
    MarketList,
    /// Get orderbook for a symbol at a certain level
    OrderBook(OrderBook),
    /// Get trade histories for a symbol
    TradeHistories(TradeHistories),
    /// Get klines for a symbol
    Klines(KlinesClap),
    /// Get currencies
    Currencies,
    /// Get fiat prices
    FiatPrices(FiatPrices),
    /// Get server time
    ServerTime,
}

#[derive(Parser, Debug)]
pub struct Symbols {
    #[arg(short, long)]
    market: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Ticker {
    symbol: String,
}

#[derive(Parser, Debug)]
pub struct DailyStats {
    symbol: String,
}

#[derive(Parser, Debug)]
pub struct OrderBook {
    symbol: String,
    #[arg(value_parser = OrderBookType::VARIANTS, num_args = 1)]
    amount: String,
}

#[derive(Parser, Debug)]
pub struct TradeHistories {
    symbol: String,
}

#[derive(Parser, Debug)]
pub struct KlinesClap {
    #[arg(value_parser = Klines::VARIANTS, num_args = 1)]
    klines: String,
    symbol: String,
    #[arg(short, long, value_parser = dateparser::parse)]
    start_at: Option<DateTime<Utc>>,
    #[arg(short, long, value_parser = dateparser::parse)]
    end_at: Option<DateTime<Utc>>,
}

#[derive(Parser, Debug)]
pub struct FiatPrices {
    #[arg(short, long)]
    base: Option<String>,
    #[arg(short, long)]
    currencies: Option<String>,
}

pub async fn process_market(command: Market) -> Result<()> {
    let client = Kucoin::new(KucoinEnv::Live, None)?;
    match command {
        Market::Symbols(symbols) => {
            let symbols = client.get_symbol_list(symbols.market.as_deref()).await?;
            println!("{symbols}");
        }
        Market::Ticker(ticker) => {
            let ticker = client.get_ticker(&ticker.symbol).await?;
            println!("{ticker}");
        }
        Market::AllTickers => {
            let all_tickers = client.get_all_tickers().await?;
            println!("{all_tickers}");
        }
        Market::DailyStats(daily_stats) => {
            let daily_stats = client.get_daily_stats(&daily_stats.symbol).await?;
            println!("{daily_stats}");
        }
        Market::MarketList => {
            let market_list = client.get_market_list().await?;
            println!("{market_list}");
        }
        Market::OrderBook(order_book) => {
            let order_book = client
                .get_orderbook(&order_book.symbol, order_book.amount.parse()?)
                .await?;
            println!("{order_book}");
        }
        Market::TradeHistories(trade_histories) => {
            let trade_histories = client.get_trade_histories(&trade_histories.symbol).await?;
            println!("{trade_histories}");
        }
        Market::Klines(klines) => {
            let klines_str = klines.klines.parse()?;
            let start_at = klines.start_at.map(|d| d.timestamp() as u64);
            let end_at = klines.end_at.map(|d| d.timestamp() as u64);
            let request = CandleRequest::new()
            let klines = client
                .get_klines(klines_str, &klines.symbol, start_at, end_at)
                .await?;
            println!("{klines}");
        }
        Market::Currencies => {
            let currencies = client.get_currencies().await?;
            println!("{currencies}");
        }
        Market::FiatPrices(fiat_prices) => {
            let fiat_prices = client
                .get_fiat_prices(fiat_prices.base, fiat_prices.currencies)
                .await?;
            println!("{fiat_prices}");
        }
        Market::ServerTime => {
            let server_time = client.get_server_time().await?;
            println!("{server_time}");
        }
    }
    Ok(())
}
