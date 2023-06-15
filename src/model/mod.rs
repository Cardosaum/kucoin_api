//! All Kucoin API endpoint response objects

use serde::Serialize;
pub mod margin;
pub mod market;
pub mod trade;
pub mod user;
pub mod websocket;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIData<T> {
    pub code: String,
    pub data: Option<Vec<T>>,
    pub msg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIDatum<T> {
    pub code: String,
    pub data: Option<T>,
    pub msg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T> {
    pub current_page: i32,
    pub page_size: i32,
    pub total_num: i32,
    pub total_page: i32,
    pub items: Vec<T>,
}

impl<T> std::fmt::Display for APIData<T>
where
    T: Serialize
 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl<T> std::fmt::Display for APIDatum<T>
where
    T: Serialize
 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl<T> std::fmt::Display for Pagination<T>
where
    T: Serialize
 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}   