use std::collections::HashMap;

use super::client::Kucoin;
use super::model::margin::BorrowOrder;
use super::model::margin::BorrowOrderId;
use super::model::margin::LendHistory;
use super::model::margin::LendMarketData;
use super::model::margin::LendOrder;
use super::model::margin::LendRecord;
use super::model::margin::MarginAccounts;
use super::model::margin::MarginHistory;
use super::model::margin::MarginInfo;
use super::model::margin::MarginOrder;
use super::model::margin::MarginOrderId;
use super::model::margin::MarginTradeData;
use super::model::margin::MarkPrice;
use super::model::margin::RepayRecord;
use super::model::margin::RepaymentRecord;
use super::model::APIData;
use super::model::APIDatum;
use super::model::Method;
use super::model::Pagination;
use super::utils::format_query;
use crate::error::Result;

impl Kucoin {
    pub async fn get_mark_price(&self, symbol: &str) -> Result<APIDatum<MarkPrice>> {
        let endpoint = format!("/api/v1/mark-price/{}/current", symbol);
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_margin_config_info(&self) -> Result<APIDatum<MarginInfo>> {
        let endpoint = String::from("/api/v1/margin/config");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_margin_accounts(&self) -> Result<APIDatum<MarginAccounts>> {
        let endpoint = String::from("/api/v1/margin/account");
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self.sign_headers(endpoint, None, None, Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    /// Term param is comma delimited. Avaialble terms are 7,14,28
    pub async fn post_borrow_order(
        &self,
        currency: &str,
        trade_type: &str,
        size: f64,
        max_rate: Option<f64>,
        term: Option<&str>,
    ) -> Result<APIDatum<BorrowOrderId>> {
        let endpoint = String::from("/api/v1/margin/borrow");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("type"), trade_type.to_string());
        params.insert(String::from("size"), size.to_string());
        if let Some(m) = max_rate {
            params.insert(String::from("maxRate"), m.to_string());
        }
        if let Some(t) = term {
            params.insert(String::from("term"), t.to_string());
        }
        let headers = self.sign_headers(endpoint, Some(&params), None, Method::POST).unwrap();
        let resp = self.post(url, Some(headers), Some(params)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_borrow_order(&self, order_id: &str) -> Result<APIDatum<BorrowOrder>> {
        let endpoint = format!("/api/v1/margin/borrow?orderId={}", order_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self.sign_headers(endpoint, None, None, Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_repay_record(
        &self,
        currency: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<RepayRecord>>> {
        let endpoint = String::from("/api/v1/margin/borrow/outstanding");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self.sign_headers(endpoint, None, Some(query), Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_repayment_record(
        &self,
        currency: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<RepaymentRecord>>> {
        let endpoint = String::from("/api/v1/margin/borrow/repaid");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self.sign_headers(endpoint, None, Some(query), Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn one_click_repayment(&self, currency: &str, sequence: &str, size: f64) -> Result<APIDatum<String>> {
        let endpoint = String::from("/api/v1/margin/repay/all");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("sequence"), sequence.to_string());
        params.insert(String::from("size"), size.to_string());
        let headers = self.sign_headers(endpoint, Some(&params), None, Method::POST).unwrap();
        let resp = self.post(url, Some(headers), Some(params)).await?.json().await?;
        Ok(resp)
    }

    pub async fn repay_single_order(&self, currency: &str, trade_id: &str, size: f64) -> Result<APIDatum<String>> {
        let endpoint = String::from("/api/v1/margin/repay/single");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("trade_id"), trade_id.to_string());
        params.insert(String::from("size"), size.to_string());
        let headers = self.sign_headers(endpoint, Some(&params), None, Method::POST).unwrap();
        let resp = self.post(url, Some(headers), Some(params)).await?.json().await?;
        Ok(resp)
    }

    pub async fn post_lend_order(
        &self,
        currency: &str,
        size: f32,
        daily_int_rate: f32,
        term: i32,
    ) -> Result<APIDatum<MarginOrderId>> {
        let endpoint = String::from("/api/v1/margin/lend");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("size"), size.to_string());
        params.insert(String::from("dailyIntRate"), daily_int_rate.to_string());
        params.insert(String::from("term"), term.to_string());
        let headers = self.sign_headers(endpoint, Some(&params), None, Method::POST).unwrap();
        let resp = self.post(url, Some(headers), Some(params)).await?.json().await?;
        Ok(resp)
    }

    pub async fn cancel_lend_order(&self, order_id: &str) -> Result<APIDatum<String>> {
        let endpoint = format!("/api/v1/margin/lend/{}", order_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self.sign_headers(endpoint, None, None, Method::DELETE).unwrap();
        let resp = self.delete(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn set_auto_lend(
        &self,
        currency: &str,
        is_enable: bool,
        retain_size: Option<f32>,
        daily_int_rate: Option<f32>,
        term: Option<i32>,
    ) -> Result<APIDatum<String>> {
        let endpoint = String::from("/api/v1/margin/toggle-auto-lend");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("isEnable"), is_enable.to_string());
        if let Some(r) = retain_size {
            params.insert(String::from("retainSize"), r.to_string());
        }
        if let Some(d) = daily_int_rate {
            params.insert(String::from("dailyIntRate"), d.to_string());
        }
        if let Some(t) = term {
            params.insert(String::from("term"), t.to_string());
        }
        let headers = self.sign_headers(endpoint, Some(&params), None, Method::POST).unwrap();
        let resp = self.post(url, Some(headers), Some(params)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_active_order(
        &self,
        currency: &str,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<MarginOrder>>> {
        let endpoint = String::from("/api/v1/margin/lend/active");
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self.sign_headers(endpoint, None, Some(query), Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_lend_history(
        &self,
        currency: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<MarginHistory>>> {
        let endpoint = String::from("/api/v1/margin/lend/done");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self.sign_headers(endpoint, None, Some(query), Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_active_lend(
        &self,
        currency: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<LendOrder>>> {
        let endpoint = String::from("/api/v1/margin/lend/trade/unsettled");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self.sign_headers(endpoint, None, Some(query), Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_settled_lend(
        &self,
        currency: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<LendHistory>>> {
        let endpoint = String::from("/api/v1/margin/lend/trade/settled");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self.sign_headers(endpoint, None, Some(query), Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_lend_record(&self, currency: Option<&str>) -> Result<APIData<LendRecord>> {
        let mut endpoint = String::from("/api/v1/margin/lend/assets");
        if let Some(c) = currency {
            endpoint.push_str(&format!("?currency={}", c));
        }
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self.sign_headers(endpoint, None, None, Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_lend_market_data(&self, currency: &str, term: Option<i32>) -> Result<APIData<LendMarketData>> {
        let mut endpoint = format!("/api/v1/margin/market?currency={}", currency);
        if let Some(t) = term {
            endpoint.push_str(&format!("term={}", t));
        }
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self.sign_headers(endpoint, None, None, Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_margin_trade_data(&self, currency: &str) -> Result<APIData<MarginTradeData>> {
        let endpoint = format!("/api/v1/margin/trade/last?currency={}", currency);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self.sign_headers(endpoint, None, None, Method::GET).unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }
}
