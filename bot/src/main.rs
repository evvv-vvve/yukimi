use discord_event_handler::Handler;
use serenity::framework::StandardFramework;
use serenity::prelude::*;

use config::Config;
use commands::*;

pub mod config;
pub mod commands;
pub mod discord_event_handler;

#[tokio::main]
async fn main() {match Config::from_file("config.toml") {
    Ok(config) => {
      let framework = StandardFramework::new()
         .configure(|conf| conf.prefixes(config.get_prefixes()))
         .group(&GENERAL_GROUP);

      let intents = GatewayIntents::non_privileged()
         | GatewayIntents::MESSAGE_CONTENT
         | GatewayIntents::DIRECT_MESSAGES
         | GatewayIntents::GUILD_MESSAGES;

      let mut client = Client::builder(config.get_token(), intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't create client!");

      if let Err(err) = client.start().await {
        println!("Error while running client: {err:?}")
      }
    },
    Err(config_error) => {
      println!("An error occurred while loading config: {config_error:#}")
    }
  }
}
