use super::APIResult;
use chrono::prelude::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub date: DateTime<Local>,
    pub dishes: Vec<String>,
    pub id: String,
}

impl Menu {
    fn extract_date(element: &scraper::ElementRef) -> DateTime<Local> {
        let selector = Selector::parse(".panel-heading > .pull-right").unwrap();
        let date_str = element.select(&selector).next().unwrap().inner_html();

        let mut segments = date_str.split_whitespace();

        let day = segments.next().unwrap().parse::<u32>().unwrap();

        let month_str = segments.next().unwrap();

        let month = match month_str {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "maj" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "okt" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => unreachable!(),
        };

        let now = Local::now();

        // `ymd` does not like zero-indexed months.
        let intermediate = Local.ymd(now.year(), month, day).and_hms(0, 0, 0);

        if intermediate < now {
            return intermediate.with_year(now.year() + 1).unwrap();
        }

        intermediate
    }

    fn extract_dishes(element: &scraper::ElementRef) -> Vec<String> {
        let selector =
            Selector::parse(".list-group > .list-group-item > div.app-daymenu-name").unwrap();

        let dishes: Vec<String> = element
            .select(&selector)
            .map(|element| element.inner_html().trim().to_string())
            .collect();

        dishes
    }

    pub fn from_element(element: scraper::ElementRef) -> Option<Self> {
        let id_selector = Selector::parse(".list-group > .list-group-item > .icon-left").unwrap();

        let id_element = match element.select(&id_selector).next() {
            Some(id_element) => id_element,
            None => return None,
        };

        Some(Self {
            dishes: Self::extract_dishes(&element),
            date: Self::extract_date(&element),
            id: id_element.value().attr("js-meal-id").unwrap().to_string(),
        })
    }

    pub async fn scrape(url: String) -> APIResult<Vec<Menu>> {
        let response = reqwest::get(&url).await?;
        let html = response.text().await?;

        let document = Html::parse_document(&html);
        let selector = Selector::parse("#app-page .panel").unwrap();

        let menus = document
            .select(&selector)
            .filter_map(Menu::from_element)
            .collect();

        Ok(menus)
    }
}
