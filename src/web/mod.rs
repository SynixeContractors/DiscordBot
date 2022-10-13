use std::sync::Arc;

use actix_web::{
    web::{self, Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use serenity::{model::prelude::ReactionType, CacheAndHttp};

mod state;
use state::State;

use crate::bot::{DISCORD_CHANNEL_PLANNING, DISCORD_CHANNEL_SCHEDULE, DISCORD_GUILD_SYNIXE};

pub fn start(http: Arc<CacheAndHttp>) -> actix_web::dev::Server {
    let state = State { bot: http };
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .route("/members", web::get().to(members))
            .route("/schedule", web::post().to(schedule))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
}

async fn members(data: Data<State>) -> impl Responder {
    let guild = data
        .bot
        .http
        .get_guild_members(DISCORD_GUILD_SYNIXE.0, None, None)
        .await
        .unwrap();
    HttpResponse::Ok().json(guild)
}

#[derive(Deserialize)]
pub struct SchedulePost {
    name: String,
    date: String,
    summary: String,
    description: String,
}

async fn schedule(data: Data<State>, post: Json<SchedulePost>) -> impl Responder {
    match data.bot.http.send_message(DISCORD_CHANNEL_SCHEDULE.0, &serde_json::json!({
        "content": format!{"**{}**\n<t:{}:F> - <t:{}:R>\n\n{}", post.name, post.date, post.date, post.summary},
    })).await {
        Ok(message) => {
            for reaction in ["🟩", "🟨", "🟥"].iter() {
                let _ = message.react(&data.bot.http, ReactionType::Unicode(reaction.to_string())).await;
            }
            match data.bot.http.create_public_thread(DISCORD_CHANNEL_SCHEDULE.0, message.id.0, &{
                let mut map = serde_json::Map::new();
                map.insert("name".to_string(), serde_json::Value::String(format!("{} - Preferred Roles", post.name)));
                map
            }).await {
                Ok(_) => {
                    match data.bot.http.send_message(DISCORD_CHANNEL_PLANNING.0, &serde_json::json!({
                        "content": format!{"**{}**\n<t:{}:F> - <t:{}:R>", post.name, post.date, post.date},
                    })).await {
                        Ok(planning_message) => {
                            if let Ok(planning_thread) = data.bot.http.create_public_thread(DISCORD_CHANNEL_PLANNING.0, planning_message.id.0, &{
                                let mut map = serde_json::Map::new();
                                map.insert("name".to_string(), serde_json::Value::String(post.name.clone()));
                                map
                            }).await {
                                let _ = data.bot.http.send_message(planning_thread.id.0, &serde_json::json!({
                                    "content": post.description,
                                })).await;
                            }
                            HttpResponse::Ok().finish()
                        }
                        Err(e) => {
                            println!("Error creating planning thread: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(e) => {
                    println!("Error creating planning thread: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
