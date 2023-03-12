use anyhow::anyhow;
use serenity::async_trait;

use serenity::builder::CreateApplicationCommandOption;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::gateway::Ready;
use serenity::model::prelude::interaction::Interaction::ApplicationCommand;
use serenity::model::prelude::*;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;

mod controllers;
mod models;
mod services;

use controllers::category_controller::CategoryController;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = GuildId(1044972423587561504);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("new_category")
                        .description("add new log category.")
                        .create_option(|option| {
                            option
                                .name("name")
                                .kind(command::CommandOptionType::String)
                                .description("category's name.")
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("get_categories")
                        .description("get all categories.")
                })
        })
        .await
        .unwrap();
    }

    async fn interaction_create(
        &self,
        ctx: Context,
        interaction: serenity::model::application::interaction::Interaction,
    ) {
        if let ApplicationCommand(command) = interaction {
            let result = match command.data.name.as_str() {
                "new_category" => {
                    println!(
                        "{:?}",
                        &command.data.options[0].value.clone().unwrap().to_string()
                    );
                    CategoryController::add_category(
                        command.data.options[0].value.clone().unwrap().to_string(),
                    )
                    .await
                }
                "get_categories" => Ok(CategoryController::get_categories()
                    .await
                    .unwrap()
                    .iter()
                    .fold("".to_string(), |acc, category| {
                        (acc + &category.name + "\n").to_string()
                    })
                    .to_string()),

                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(match result {
                                Ok(message) => message,
                                Err(e) => e.to_string(),
                            })
                        })
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
