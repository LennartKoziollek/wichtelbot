use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, builder::CreateApplicationCommand};

use crate::data::functions::add_user;

pub fn run(command: &ApplicationCommandInteraction) -> String {
    let id = &command.member.as_ref().unwrap().user.id;
    let result = add_user(*id.as_u64()); 
    match result {
        true => "Joined the wichteln!".to_string(),
        false => "Could not join the wichteln...".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("Join the wichteln!")
}