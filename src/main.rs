extern crate dotenv;

use dotenv::dotenv;
use std::env;
use regex::Regex;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub mod helpers;
use helpers::handle_ratio::RatioHandler;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let re: Regex = Regex::new(r#"(?:^|\\w)ratio+(?:$|\\w)|counter+(?:$|\\w)"#).unwrap();
        match re.captures(&msg.content) {
            Some(_) => {
                let rand_val = RatioHandler::gen_random();
                if rand_val == 0 {
                    // Accepted Ratio/Counter
                    msg.react(&ctx, '👍').await.expect("Couldnt react!");
                    if let Err(why) = msg.reply_ping(&ctx.http, rand_val).await {
                        println!("{:?}", why);
                    }
                    
                }
                else {
                    // Denied Ratio/Counter
                    msg.react(&ctx, '👎').await.expect("Couldnt react!");
                    if let Err(why) = msg.reply_ping(&ctx.http, rand_val).await {
                        println!("{:?}", why);
                    }
                }
                // if let Err(why) = msg.channel_id.say(&ctx.http, rand_val).await {
                //     println!("{:?}", why);
                // }
            }
            None => ()
        }
        // if re.is_match(&msg.content.to_lowercase()) {
            
        //     if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        //         println!("Error sending message: {:?}", why);
        //     }
        // }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISC_TOKEN").expect("Expected a token in the enviornment!");

    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}