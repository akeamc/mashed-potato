use super::APIResult;
use chrono::prelude::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use itertools::Itertools;

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
    pub date: DateTime<Utc>,
    pub dishes: Vec<Dish>,
}

impl Menu {
    fn extract_date(element: &scraper::ElementRef) -> Option<DateTime<Utc>> {
        let selector = Selector::parse(".panel-heading > .pull-right").unwrap();
        let date_str = element.select(&selector).next().unwrap().inner_html();

        let mut segments = date_str.split_whitespace();

        let day = segments
            .next()
            .and_then(|day_str| day_str.parse::<u32>().ok());

        let month = segments.next().and_then(|month_str| match month_str {
            "jan" => Some(1),
            "feb" => Some(2),
            "mar" => Some(3),
            "apr" => Some(4),
            "maj" => Some(5),
            "jun" => Some(6),
            "jul" => Some(7),
            "aug" => Some(8),
            "sep" => Some(9),
            "okt" => Some(10),
            "nov" => Some(11),
            "dec" => Some(12),
            _ => None,
        });

        match (day, month) {
            (Some(day), Some(month)) => Some(
                chrono_tz::Europe::Stockholm
                    .ymd(Local::now().year(), month, day)
                    .and_hms(0, 0, 0)
                    .with_timezone(&Utc),
            ),
            _ => None,
        }
    }

    pub fn from_element(element: scraper::ElementRef) -> Option<Self> {
        let dish_selector = Selector::parse(".list-group > .list-group-item").unwrap();

        let dishes: Vec<Dish> = element
            .select(&dish_selector)
            .filter_map(Dish::from_element)
            .collect();

        let date = Self::extract_date(&element);

        match (date, dishes.is_empty()) {
            (Some(date), false) => Some(Self { dishes, date }),
            _ => None,
        }
    }

    pub async fn scrape(url: String) -> APIResult<Vec<Menu>> {
        let response = reqwest::get(&url).await?;
        let html = response.text().await?;

        let document = Html::parse_document(&html);
        let selector = Selector::parse("#app-page .panel").unwrap();

        let menus = document
            .select(&selector)
            .filter_map(Menu::from_element)
            .dedup_by(|a, b| a.date == b.date)
            .collect();

        Ok(menus)
    }
}
