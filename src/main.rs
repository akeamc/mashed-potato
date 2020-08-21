mod api;

#[tokio::main]
async fn main() {
    let search_results = api::search("Södermalmsskolan".to_string()).await.unwrap();

    for result in search_results {
        let menus = api::scrape_menus(result.url()).await.unwrap();
        println!("{:?}", menus);
    }
}
