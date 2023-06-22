use std::collections::HashMap;

use reqwest::header;

use crate::client::Kucoin;
use crate::error::Result;
use crate::model::market::AllTickers;
use crate::model::market::Candle;
use crate::model::market::Chain;
use crate::model::market::Currency;
use crate::model::market::DailyStats;
use crate::model::market::OrderBook;
use crate::model::market::OrderBookType;
use crate::model::market::SymbolList;
use crate::model::market::Ticker;
use crate::model::market::TradeHistories;
use crate::model::APIData;
use crate::model::APIDatum;
use crate::model::Method;
use crate::utils::format_query;

impl Kucoin {
    pub async fn get_symbol_list(&self, market: Option<&str>) -> Result<APIData<SymbolList>> {
        // V1 is deprecated already
        let endpoint = String::from("/api/v2/symbols");
        let url = match market {
            Some(m) => format!("{}{}?market={}", &self.prefix, endpoint, m),
            None => format!("{}{}", &self.prefix, endpoint),
        };
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_ticker(&self, symbol: &str) -> Result<APIDatum<Ticker>> {
        let endpoint = String::from("/api/v1/market/orderbook/level1");
        let url = format!("{}{}?symbol={}", &self.prefix, endpoint, symbol);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_all_tickers(&self) -> Result<APIDatum<AllTickers>> {
        let endpoint = String::from("/api/v1/market/allTickers");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_daily_stats(&self, symbol: &str) -> Result<APIDatum<DailyStats>> {
        let endpoint = String::from("/api/v1/market/stats");
        let url = format!("{}{}?symbol={}", &self.prefix, endpoint, symbol);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_market_list(&self) -> Result<APIData<String>> {
        let endpoint = String::from("/api/v1/markets");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_orderbook(&self, symbol: &str, amount: OrderBookType) -> Result<APIDatum<OrderBook>> {
        let endpoint = match amount {
            OrderBookType::L20 => format!("/api/v1/market/orderbook/level2_20?symbol={}", symbol),
            OrderBookType::L100 => format!("/api/v1/market/orderbook/level2_100?symbol={}", symbol),
            OrderBookType::Full => format!("/api/v3/market/orderbook/level2?symbol={}", symbol),
        };
        match amount {
            OrderBookType::L20 | OrderBookType::L100 => {
                let url = format!("{}{}", &self.prefix, endpoint);
                let resp: APIDatum<OrderBook> = self.get(url, None).await?.json().await?;
                Ok(resp)
            },
            OrderBookType::Full => {
                let url = format!("{}{}", &self.prefix, endpoint);
                let headers: header::HeaderMap = self.sign_headers(endpoint, None, None, Method::GET).unwrap();
                let resp = self.get(url, Some(headers)).await?.json().await?;
                Ok(resp)
            },
        }
    }

    pub async fn get_trade_histories(&self, symbol: &str) -> Result<APIData<TradeHistories>> {
        let endpoint = format!("/api/v1/market/histories?symbol={}", symbol);
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_klines(&self, request: crate::model::request::market::CandleRequest) -> Result<APIData<Candle>> {
        let endpoint = request.get_endpoint();
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_currencies(&self) -> Result<APIData<Currency>> {
        let endpoint = String::from("/api/v1/currencies");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_currency(&self, currency: &str, chain: Option<Chain>) -> Result<APIDatum<Currency>> {
        let mut endpoint = format!("/api/v1/currencies/{}", currency);
        if let Some(c) = chain {
            endpoint.push_str(&format!("?chain={c}"));
        }
        let url = format!("{}{endpoint}", &self.prefix);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_fiat_prices(
        &self,
        base: Option<impl ToString>,
        currencies: Option<impl ToString>,
    ) -> Result<APIDatum<HashMap<String, String>>> {
        let endpoint = String::from("/api/v1/prices");
        let mut params: HashMap<String, String> = HashMap::new();
        let url: String;
        if let Some(b) = base {
            params.insert(String::from("base"), b.to_string());
        }
        if let Some(c) = currencies {
            params.insert(String::from("currencies"), c.to_string());
        }
        if !params.is_empty() {
            let query = format_query(&params);
            url = format!("{}{}{}", &self.prefix, endpoint, query);
        } else {
            url = format!("{}{}", &self.prefix, endpoint);
        }
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_server_time(&self) -> Result<APIDatum<i64>> {
        let endpoint = String::from("/api/v1/timestamp");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }
}
