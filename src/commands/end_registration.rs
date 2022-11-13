use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

pub fn run(_options: &[CommandDataOption]) -> String {
    "end_registration".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("end_registration")
        .description("end the registration period and send wichtel their bewichtelter")
}
