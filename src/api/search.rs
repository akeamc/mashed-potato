use super::APIResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub id: String,
    pub title: String,

    #[serde(alias = "url")]
    pub path: String,
}

impl SearchResult {
    pub fn url(&self) -> String {
        format!("https://sodexo.mashie.com{}", self.path)
    }
}

pub async fn search(query: String) -> APIResult<Vec<SearchResult>> {
    let mut body = HashMap::new();
    body.insert("query", query);

    let client = reqwest::Client::new();
    let response = client
        .post("https://sodexo.mashie.com/public/app/internal/execute-query?country=se")
        .json(&body)
        .send()
        .await?;

    let results = response.json::<Vec<SearchResult>>().await?;

    Ok(results)
}
