use super::{APIResult, Menu};
use scraper::Selector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dish {
    pub title: String,
    pub id: String,

    co2e_url: String,

    /// CO₂ equivalents expressed in kilograms per dish.
    pub co2e: Option<f64>,
}

/// Emissions rating found on [sodexo.mashie.com](https://sodexo.mashie.com).
#[derive(Serialize, Deserialize, Debug)]
pub struct DishEmissionsRating {
    pub score: u8,

    /// CO₂ equivalents expressed in kilograms per dish.
    #[serde(rename = "kgCo2E")]
    pub co2e: f64,

    #[serde(rename = "imageUrl")]
    pub image_url: String,
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

    fn extract_co2_url(element: &scraper::ElementRef) -> Option<String> {
        let selector = Selector::parse(".modal .modal-content .modal-body  .well").unwrap();

        element
            .select(&selector)
            .next()
            .map(|element| {
                element
                    .value()
                    .attr("js-load-rating")
                    .map(|attr| format!("https://sodexo.mashie.com{}", attr.to_string()))
            })
            .flatten()
    }

    pub fn from_element(element: scraper::ElementRef) -> Option<Self> {
        let id = Self::extract_id(&element)?;
        let title = Self::extract_title(&element)?;
        let co2e_url = Self::extract_co2_url(&element)?;

        Some(Self {
            id,
            title,
            co2e_url,
            co2e: None,
        })
    }

    pub async fn fetch_co2e(&mut self) -> Result<(), reqwest::Error> {
        let response = reqwest::get(&self.co2e_url).await?;
        let rating = response.json::<DishEmissionsRating>().await?;

        self.co2e = Some(rating.co2e);

        Ok(())
    }

    async fn fetch_map(url: String) -> APIResult<HashMap<String, Dish>> {
        let menu = Menu::scrape(url).await?;

        let map: HashMap<String, Dish> =
            menu.into_iter()
                .fold(HashMap::<String, Dish>::new(), |mut acc, day| {
                    for dish in day.dishes.into_iter() {
                        acc.insert(dish.id.clone(), dish);
                    }

                    acc
                });

        Ok(map)
    }

    pub async fn fetch_all(url: String) -> APIResult<Vec<Self>> {
        let map = Self::fetch_map(url).await?;

        let dishes: Vec<Self> = map.into_iter().map(|(_id, dish)| dish).collect();

        Ok(dishes)
    }

    pub async fn fetch(url: String, id: &str) -> APIResult<Option<Self>> {
        let mut dishes = Self::fetch_map(url).await?;

        match dishes.remove(id) {
            Some(mut dish) => {
                dish.fetch_co2e().await?;

                Ok(Some(dish))
            }
            None => Ok(None),
        }
    }
}
