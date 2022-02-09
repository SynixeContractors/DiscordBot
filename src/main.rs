//! Discord bot for Synixe Contractors Discord server.

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::async_trait;

const DISCORD_CHANNEL_LOBBY: u64 = 700_888_247_928_356_908;
const DISCORD_CHANNEL_LOG: u64 = 700_943_290_102_448_208;
const DISCORD_GUILD_SYNIXE: u64 = 700_888_247_928_356_905;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, _ctx: Context, guild_id: GuildId, new_member: Member) {
        if new_member.user.bot {
            println!("Skipping bot");
            return;
        }
        if guild_id.0 == DISCORD_GUILD_SYNIXE {
            ChannelId(DISCORD_CHANNEL_LOBBY)
                .send_message(&_ctx, |m| {
                    m.content(&format!(
                        "Welcome <@{}>! Please follow the steps in <#700888595850068101> to get prepared to jump in game with us. If you have any questions, feel free to ask here!",
                        new_member.user.id
                    ))
                })
                .await
                .unwrap();
        }
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        kicked: User,
        _member: Option<Member>,
    ) {
        if kicked.bot {
            println!("Skipping bot");
            return;
        }
        if guild_id.0 == DISCORD_GUILD_SYNIXE {
            ChannelId(DISCORD_CHANNEL_LOG)
                .send_message(&ctx, |m| {
                    m.content(&format!(
                        "{}#{} ({}) has left, <@{}>",
                        kicked.name, kicked.discriminator, kicked.id, kicked.id
                    ))
                })
                .await
                .unwrap();
        }
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        if banned_user.bot {
            println!("Skipping bot");
            return;
        }
        if guild_id.0 == DISCORD_GUILD_SYNIXE {
            ChannelId(DISCORD_CHANNEL_LOG)
                .send_message(&ctx, |m| {
                    m.content(&format!(
                        "{}#{} ({}) was banned, <@{}>",
                        banned_user.name, banned_user.discriminator, banned_user.id, banned_user.id
                    ))
                })
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .intents(
            GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS,
        )
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
