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

fn parse_date(date_string: String) -> DateTime<Local> {
    let mut segments = date_string.split_whitespace();

    let day = segments.next().unwrap().parse::<u32>().unwrap();

    let month_str = segments.next().unwrap();

    let month = match month_str {
        "jan" => 0,
        "feb" => 1,
        "mar" => 2,
        "apr" => 3,
        "maj" => 4,
        "jun" => 5,
        "jul" => 6,
        "aug" => 7,
        "sep" => 8,
        "okt" => 9,
        "nov" => 10,
        "dec" => 11,
        _ => unreachable!(),
    };

    let now = Local::now();

    // `ymd` does not like zero-indexed months.
    let intermediate = Local.ymd(now.year(), month + 1, day).and_hms(0, 0, 0);

    if intermediate < now {
        return intermediate.with_year(now.year() + 1).unwrap();
    }

    intermediate
}

fn parse_element(element: scraper::ElementRef) -> Option<Menu> {
    let date_selector = Selector::parse(".panel-heading > .pull-right").unwrap();
    let dish_selector =
        Selector::parse(".list-group > .list-group-item > div.app-daymenu-name").unwrap();
    let id_selector = Selector::parse(".list-group > .list-group-item > .icon-left").unwrap();

    let id_element = match element.select(&id_selector).next() {
        Some(id_element) => id_element,
        None => return None,
    };

    let date_str = element.select(&date_selector).next().unwrap().inner_html();
    let date = parse_date(date_str);

    Some(Menu {
        dishes: element
            .select(&dish_selector)
            .map(|element| element.inner_html())
            .collect(),
        date,
        id: id_element.value().attr("js-meal-id").unwrap().to_string(),
    })
}

pub async fn scrape_menus(url: String) -> APIResult<Vec<Menu>> {
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse("#app-page .panel").unwrap();

    let menus = document
        .select(&selector)
        .filter_map(parse_element)
        .collect();

    Ok(menus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_parsing() {
        let parsed = parse_date("24 dec".to_string());

        assert_eq!(parsed.month(), 11);
        assert_eq!(parsed.day(), 24);
    }
}
