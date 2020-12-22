use super::{APIError, APIResult};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StudySet {
    pub id: String,
    pub title: String,
    pub description: String,
    pub term_count: u32,
    pub author: String,
}

impl StudySet {
    pub fn get_url(id: &str) -> String {
        format!("https://quizlet.com/{}/bruh", id)
    }

    fn extract_title(document: &Html) -> Option<String> {
        let selector = Selector::parse(".UIHeading--one").unwrap();

        document
            .select(&selector)
            .next()
            .map(|element| element.inner_html())
    }

    fn extract_description(document: &Html) -> Option<String> {
        let selector = Selector::parse(".SetPageHeader-description").unwrap();

        document
            .select(&selector)
            .next()
            .map(|element| element.inner_html())
    }

    fn extract_term_count(document: &Html) -> Option<u32> {
        let selector = Selector::parse(".UIHeading.UIHeading--four").unwrap();

        document
            .select(&selector)
            .next()
            .map(|element| {
                let text = element.inner_html();

                text.chars()
                    .skip_while(|ch| !ch.is_digit(10))
                    .take_while(|ch| ch.is_digit(10))
                    .fold(None, |acc, ch| {
                        ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
                    })
            })
            .flatten()
    }

    fn extract_author(document: &Html) -> Option<String> {
        let selector = Selector::parse(".UserLink-username").unwrap();

        document
            .select(&selector)
            .next()
            .map(|element| element.inner_html())
    }

    fn extract(document: &Html, id: String) -> Option<Self> {
        let title = Self::extract_title(&document)?;
        let description = Self::extract_description(&document).unwrap_or_else(|| "".to_string());
        let term_count = Self::extract_term_count(&document)?;
        let author = Self::extract_author(&document)?;

        Some(Self {
            id,
            title,
            description,
            term_count,
            author,
        })
    }

    pub async fn scrape(id: String) -> APIResult<StudySet> {
        let client = reqwest::Client::new();

        let response = client
            .get(&Self::get_url(&id))
            .header(USER_AGENT, "POTATO")
            .send()
            .await?;
        let html = response.text().await?;

        let document = Html::parse_document(&html);

        Self::extract(&document, id)
            .ok_or_else(|| APIError::NotFound("Study set not found".to_string()))
    }
}
