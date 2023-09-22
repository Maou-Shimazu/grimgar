use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Permissions;
use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        Timestamp,
    },
    prelude::Context,
    utils::Colour,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("message")
        .description("Send a moderator message.")
        .default_member_permissions(Permissions::MANAGE_GUILD)
        .create_option(|channel| {
            channel
                .name("channel")
                .description("Channel to send message in")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_option(|message| {
            message
                .name("message")
                .description("message to send")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn message(ctx: &Context, command: &ApplicationCommandInteraction) {
    // refractor to use dashboard
    let channel = command
        .data
        .options
        .get(0)
        .expect("Expected channel.")
        .resolved
        .as_ref()
        .expect("Expected channel.");
    let message = command
        .data
        .options
        .get(1)
        .expect("Expected message")
        .resolved
        .as_ref()
        .expect("Expected message");

    let mem = command.member.clone().unwrap();

    if let CommandDataOptionValue::Channel(c) = channel {
        if let CommandDataOptionValue::String(s) = message {
            c.id.send_message(&ctx.http, |msg| {
                msg.embed(|embed| {
                    embed
                        .description(s)
                        .author(|f| {
                            f.name(mem.display_name())
                                .icon_url(mem.user.avatar_url().unwrap())
                        })
                        .timestamp(Timestamp::now())
                })
            })
            .await
            .expect("could not send message");
        }
    }

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|content| {
                        content
                            .title("Successfully sent message! <:Butler:895521263974494248>")
                            .colour(Colour::DARK_GREEN)
                    })
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
