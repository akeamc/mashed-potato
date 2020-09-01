mod api;

use actix_web::{
    web::{self, Json},
    Result,
};

async fn index() -> Result<Json<Vec<api::Menu>>> {
    let search_results = api::search("Södermalmsskolan").await.unwrap();
    let search_result = search_results.get(0).unwrap();

    let menus = api::Menu::scrape(search_result.url()).await.unwrap();

    Ok(Json(menus))
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
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
    })
    .bind(addr)?
    .run()
    .await
}
