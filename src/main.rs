use reqwest;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::collections::HashMap;

use std::io::{self, BufRead};

use ansi_term::Colour;

use std::time::Instant;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("discord.gift/") {
            let token = &ctx.http.token;
            let token = token.strip_prefix("Bot ").unwrap().to_string();
            println!(
                "{}",
                Colour::RGB(255, 69, 0).paint("Nitro has been sent into a channel claiming now!")
            );
            let start = Instant::now();
            let split_url: Vec<&str> = msg.content.split("discord.gift/").collect();
            let gift_code: &str = split_url[1];
            let mut json = HashMap::new();
            json.insert("channel_id", msg.channel_id.to_string());
            let client = reqwest::Client::new();
            let res = client
                .post(
                    format!(
                        "https://discordapp.com/api/v6/entitlements/gift-codes/{}/redeem",
                        gift_code
                    )
                    .as_str(),
                )
                .json(&json)
                .header("Authorization", token)
                .header("Content-Type", "application/json")
                .header("Accept", "`application/json")
                .send()
                .await
                .expect("Error sending claim request");
            if res.status().as_u16() == 200 {
                println!("{}", Colour::Green.paint("Succesfully claimed nitro!"));
            } else {
                println!(
                    "{}",
                    Colour::Red.paint(format!(
                        "Failed to claim nitro reason: {}",
                        res.text().await.unwrap()
                    ))
                )
            }
            let dur = start.elapsed();
            println!(
                "{}",
                Colour::RGB(255, 69, 0).paint(format!("finished in {}ms", dur.as_millis()))
            );
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{}",
            Colour::RGB(255, 69, 0).paint(format!(
                "starting nitro sniping from the account: {}",
                ready.user.name
            ))
        );
    }
}

#[tokio::main]
async fn main() {
    println!(
        "{}",
        Colour::RGB(255, 69, 0).paint("Rust Nitro Sniper made by Jviguy!")
    );
    println!(
        "{}",
        Colour::RGB(255, 69, 0).paint("Please enter your token: ")
    );
    let mut token = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut token).unwrap();
    println!(
        "{}",
        Colour::RGB(255, 69, 0).paint(format!("Logging in using token: {}", token))
    );
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Failure starting client!");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
