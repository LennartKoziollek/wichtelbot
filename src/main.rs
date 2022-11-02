use std::env;

mod commands;

use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{Interaction, InteractionResponseType},
        Ready,
    },
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    //gets run when the shard/bot starts
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
