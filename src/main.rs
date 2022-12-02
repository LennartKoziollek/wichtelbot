use std::env;

mod commands;
mod data;
mod errors;

use data::init::init;
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
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "join" => commands::join::run(&command),
                "leave" => commands::leave::run(&command.data.options),
                "status" => commands::status::run(&command.data.options),
                "list_wichtel" => commands::list_wichtel::run(&command.data.options),
                "end_registration" => commands::end_registration::run(&command.data.options),
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

        let _ping_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await;

        let _join_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::join::register(command)
        })
        .await;

        let _leave_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::leave::register(command)
        })
        .await;

        let _status_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::status::register(command)
        })
        .await;

        let _list_wichtel_command =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::list_wichtel::register(command)
            })
            .await;

        let _end_registration_command =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::end_registration::register(command)
            })
            .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    match init() {
        true => println!("Wichtel.txt created."),
        false => println!("Wichtel.txt already exists."),
    }

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
