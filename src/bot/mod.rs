//! Discord bot for Synixe Contractors Discord server.

use serenity::{
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::{application::interaction::Interaction, prelude::*},
    prelude::*,
};

use serenity::async_trait;

pub const DISCORD_GUILD_SYNIXE: GuildId = GuildId(700_888_247_928_356_905);
pub const DISCORD_CHANNEL_LOBBY: ChannelId = ChannelId(700_888_247_928_356_908);
pub const DISCORD_CHANNEL_LOG: ChannelId = ChannelId(700_943_290_102_448_208);
pub const DISCORD_CHANNEL_SCHEDULE: ChannelId = ChannelId(700_888_805_137_318_039);
pub const DISCORD_CHANNEL_PLANNING: ChannelId = ChannelId(883_455_598_203_650_088);

mod slash;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands =
            GuildId::set_application_commands(&DISCORD_GUILD_SYNIXE, &ctx.http, |commands| {
                commands.create_application_command(|command| slash::meme::register(command))
            })
            .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "meme" => slash::meme::run(&ctx, &command).await,
                _ => {}
            }
        }
    }

    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        if new_member.user.bot {
            println!("Skipping bot");
            return;
        }
        if new_member.guild_id == DISCORD_GUILD_SYNIXE {
            DISCORD_CHANNEL_LOBBY
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
        if guild_id == DISCORD_GUILD_SYNIXE {
            DISCORD_CHANNEL_LOG
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
        if guild_id == DISCORD_GUILD_SYNIXE {
            DISCORD_CHANNEL_LOG
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

pub async fn build() -> Client {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("token");
    Client::builder(
        token,
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS,
    )
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client")
}

pub async fn start(mut client: Client) {
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
