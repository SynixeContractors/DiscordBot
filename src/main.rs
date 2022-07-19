mod bot;
mod web;

#[tokio::main]
async fn main() {
    let client = bot::build().await;
    let (web_result, _) = tokio::join!(
        web::start(client.cache_and_http.clone()),
        bot::start(client),
    );

    web_result.unwrap();
}
