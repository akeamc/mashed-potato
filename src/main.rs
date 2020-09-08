mod api;

use actix_web::{
    web::{self, Json},
    Result,
};
use api::{APIError, APIResult};
use serde::{Deserialize, Serialize};

async fn get_school() -> APIResult<api::SearchResult> {
    let mut search_results = api::search("Södermalmsskolan").await?;
    Ok(search_results.remove(0))
}

async fn menu() -> APIResult<Json<Vec<api::Menu>>> {
    let search_result = get_school().await?;

    let menus = api::Menu::scrape(search_result.url()).await?;

    Ok(Json(menus))
}

async fn dishes() -> APIResult<Json<Vec<api::Dish>>> {
    let search_result = get_school().await?;

    let dishes = api::Dish::fetch_all(search_result.url()).await?;

    Ok(Json(dishes))
}

#[derive(Serialize, Deserialize, Debug)]
struct DishRequestParams {
    id: String,
}

async fn dish(path: web::Path<DishRequestParams>) -> APIResult<Json<api::Dish>> {
    let search_result = get_school().await?;

    let dish = api::Dish::fetch(search_result.url(), &path.id).await?;

    match dish {
        Some(dish) => Ok(Json(dish)),
        None => Err(APIError::NotFound("dish not found".into())),
    }
}

async fn health() -> Result<String> {
    Ok("health ok".to_string())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use std::env;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    println!("Probing OpenSSL certificate directories");
    openssl_probe::init_ssl_cert_env_vars();

    let addr = env::var("ADDR")
        .map(|addr_str| addr_str.parse::<SocketAddr>().unwrap())
        .unwrap_or_else(|_| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080));

    println!("Binding {}", addr);

    HttpServer::new(|| {
        App::new()
            .route("/menu", web::get().to(menu))
            .route("/dishes", web::get().to(dishes))
            .route("/dishes/{id}", web::get().to(dish))
            .route("/health", web::get().to(health))
    })
    .bind(addr)?
    .run()
    .await
}
