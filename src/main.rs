extern crate serenity;

use serenity::prelude::*;
use serenity::model::*;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn on_message(&self, _: Context, msg: Message) {
        match msg.content.to_lowercase() {
            ref _x if _x.contains("good bot") => {
                if let Err(why) = msg.channel_id.say("Good teen!") {
                    println!("Error sending message: {:?}", why);
                }
            },
            ref _x if _x.contains("am i in")
                | _x.contains("i'm in")
                | _x.contains("im in") => {
                if let Err(why) = msg.channel_id.say("You're in :ok_hand:") {
                    println!("Error sending message: {:?}", why);
                }
            },
            _ => ()
        }
    }

    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
