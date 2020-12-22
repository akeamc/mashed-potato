use super::APIResult;
use super::Dish;
use chrono::prelude::*;
use itertools::Itertools;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

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
            .and_then(|day_str| day_str.parse::<u32>().ok())?;

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
        })?;

        let now = Local::now();

        let date = chrono_tz::Europe::Stockholm
            .ymd(now.year(), month, day)
            .and_hms(0, 0, 0)
            .with_timezone(&Utc);

        // This stupid "API" doesn't tell the year. We must guess.
        if date < now {
            return date.with_year(now.year() + 1);
        }

        Some(date)
    }

    pub fn from_element(element: scraper::ElementRef) -> Option<Self> {
        let dish_selector = Selector::parse(".list-group > .list-group-item").unwrap();

        let dishes: Vec<Dish> = element
            .select(&dish_selector)
            .filter_map(Dish::from_element)
            .collect();

        let date = Self::extract_date(&element)?;

        if !dishes.is_empty() {
            Some(Self { dishes, date })
        } else {
            None
        }
    }

    pub async fn scrape(url: String) -> APIResult<Vec<Menu>> {
        let response = reqwest::get(&url).await?;
        let html = response.text().await?;

        let document = Html::parse_document(&html);
        let selector = Selector::parse("#app-page .panel").unwrap();

        let mut menus: Vec<Menu> = document
            .select(&selector)
            .filter_map(Menu::from_element)
            .collect();

        menus.sort_by(|a, b| a.date.cmp(&b.date));

        let without_duplicates: Vec<Menu> = menus
            .into_iter()
            .dedup_by(|a, b| a.date == b.date)
            .collect();

        Ok(without_duplicates)
    }
}
