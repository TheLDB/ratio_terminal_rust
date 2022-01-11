extern crate dotenv; // access .env file for secret discord token

// Serenity stuff
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

// Everything extra
use dotenv::dotenv; 
use regex::Regex; // Parse regex
use std::env;

// Helpers
pub mod helpers;
use helpers::handle_ratio::RatioHandler;

// Structs
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // fancy keysmash oooh ahhh
        let re: Regex = Regex::new(r#"(?:^|\\w)ratio+(?:$|\\w)|counter+(?:$|\\w)"#).unwrap();
        if re.is_match(&msg.content) {
            let rand_val = RatioHandler::gen_random(); 
            if rand_val == 0 {
                // Accepted Ratio/Counter
                msg.react(&ctx, '👍').await.expect("Couldnt react!");
                if let Err(why) = msg
                    .reply_ping(&ctx.http, "https://docs.idkwuu.dev/ratioaccepted.png") // have to send image link cuz serenity doesnt support reply w/ image
                    .await
                {
                    // if it fails print why
                    println!("{:?}", why)
                }
            } else {
                // Denied Ratio/Counter
                msg.react(&ctx, '👎').await.expect("Couldnt react!");
                if let Err(why) = msg
                    .reply_ping(&ctx.http, "https://docs.idkwuu.dev/ratiodeclined.png")
                    .await
                {
                    println!("{:?}", why)
                }
            }
        }
        else if msg.content == "ratio bot send code!!" {
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.add_file("src/main.rs")).await {
                println!("{:?}", why)
            }
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |f| f.add_file("src/helpers/handle_ratio.rs")).await {
                println!("{:?}", why)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISC_TOKEN").expect("Expected a token in the enviornment!");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
