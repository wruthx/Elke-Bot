use::std::env;
use::serenity::async_trait;
use::serenity::model::channel::Message;
use::serenity::model::gateway::Ready;
use::serenity::prelude::*;
use::reqwest::Client as ReqwestClient;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!strava" {
            let webhook_url = env::var("N8N_WEBHOOK_URL").expect("Expected an N8N webhook URL in the environment");

            let client = ReqwestClient::new();

            match client.post(&webhook_url).send().await {
                Ok(response) => {
                    let response_text = response.text().await.unwrap_or_else(|_| "Failed to parse the response.".to_string());

                    if let Err(why) = msg.channel_id.say(&ctx.http, response_text).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(err) => {
                    println!("Error fetching data from N8N webhook: {:?}", err);
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Failed to fetch Strava data.").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
