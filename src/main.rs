mod api;

use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct RequestQuery {
    query: String,
}

async fn index(request_query: web::Query<RequestQuery>) -> Result<web::Json<Vec<api::Menu>>> {
    let search_results = api::search(&request_query.query).await.unwrap();

    let menus = api::scrape_menus(search_results.get(0).unwrap().url())
        .await
        .unwrap();

    Ok(web::Json(menus))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
