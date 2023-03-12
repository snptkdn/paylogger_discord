use serenity::async_trait;
use anyhow::anyhow;

use serenity::builder::CreateApplicationCommandOption;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::prelude::interaction::Interaction::ApplicationCommand;
use serenity::model::application::interaction::InteractionResponseType;
use shuttle_secrets::SecretStore;

mod controllers;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = GuildId(1044972423587561504);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("hello").description("Say hello")
            }).create_application_command(|command| {
                command.name("bye").description("Say Good Bye")
            }).create_application_command(|command| {
                command
                    .name("new_category")
                    .description("add new log category.")
                    .create_option(|option|
                        option
                            .name("name")
                            .kind(command::CommandOptionType::String)
                            .description("category's name.")
                    )
            })
        })
        .await
        .unwrap();
    }

    async fn interaction_create(&self,ctx: Context,interaction: serenity::model::application::interaction::Interaction) {
        if let ApplicationCommand(command) = interaction.clone() {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello!!".to_owned(),
                "bye" => "good bye!!".to_owned(),
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
