use std::sync::Arc;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serenity::CacheAndHttp;

mod state;
use state::State;

pub fn start(http: Arc<CacheAndHttp>) -> actix_web::dev::Server {
    let state = State { bot: http };
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .route("/members", web::get().to(members))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
}

async fn members(data: Data<State>) -> impl Responder {
    let guild = data
        .bot
        .http
        .get_guild_members(crate::bot::DISCORD_GUILD_SYNIXE, None, None)
        .await
        .unwrap();
    HttpResponse::Ok().json(guild)
}
