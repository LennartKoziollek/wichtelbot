use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

pub fn run(_options: &[CommandDataOption]) -> String {
    "status".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("status").description("Check if you have joined the wichteln")
}
