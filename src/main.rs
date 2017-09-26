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

    // Set a handler to be called on the `on_ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler);

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
