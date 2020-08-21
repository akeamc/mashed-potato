use super::APIResult;
use chrono::prelude::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dish {
    pub title: String,
    pub id: String,
}

impl Dish {
    fn extract_title(element: &scraper::ElementRef) -> Option<String> {
        let selector = Selector::parse(".app-daymenu-name").unwrap();

        element
            .select(&selector)
            .next()
            .map(|element| element.inner_html())
    }

    fn extract_id(element: &scraper::ElementRef) -> Option<String> {
        let selector = Selector::parse(".icon-left").unwrap();

        element
            .select(&selector)
            .next()
            .map(|element| {
                element
                    .value()
                    .attr("js-meal-id")
                    .map(|attr| attr.to_string())
            })
            .flatten()
    }

    pub fn from_element(element: scraper::ElementRef) -> Option<Self> {
        let id = Self::extract_id(&element);
        let title = Self::extract_title(&element);

        match (id, title) {
            (Some(id), Some(title)) => Some(Self { id, title }),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub date: DateTime<Local>,
    pub dishes: Vec<Dish>,
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

    pub fn from_element(element: scraper::ElementRef) -> Option<Self> {
        let dish_selector = Selector::parse(".list-group > .list-group-item").unwrap();

        let dishes: Vec<Dish> = element
            .select(&dish_selector)
            .filter_map(Dish::from_element)
            .collect();

        if dishes.is_empty() {
            return None;
        }

        Some(Self {
            dishes,
            date: Self::extract_date(&element),
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
