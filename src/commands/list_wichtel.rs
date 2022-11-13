use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

pub fn run(_options: &[CommandDataOption]) -> String {

    "list_wichtel".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list_wichtel")
        .description("List all current wichtel")
}
