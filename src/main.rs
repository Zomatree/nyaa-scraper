use chrono::{DateTime, NaiveDateTime, Utc};
use rss::{Channel, Item};
use serde_derive::Serialize;
use std::{fs::File, io::Read, thread, time};

mod config;

#[tokio::main]
async fn main() {
    let mut config_file = File::open("config.toml").expect("No config.toml file found");
    let mut file_content = String::new();

    config_file
        .read_to_string(&mut file_content)
        .expect("Error reading config file");

    let config: config::Config = toml::from_str(&file_content).expect("Invalid toml syntax");
    let mut last_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
    let interval = time::Duration::from_secs(config.nyaa.interval);

    loop {
        let resp = reqwest::get(config.nyaa.url.clone())
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let channel = Channel::read_from(&resp[..]).unwrap();

        for item in &channel.items {
            let dt = DateTime::parse_from_rfc2822(&item.pub_date.clone().unwrap())
                .unwrap()
                .with_timezone(&Utc);
            if dt > last_dt {
                last_dt = dt;
                post_to_discord(&config, item, &dt).await;
            }
        }

        thread::sleep(interval);
    }
}
#[derive(Serialize)]
struct EmbedFooter {
    text: String,
}
#[derive(Serialize)]
struct Embed {
    r#type: String,
    title: String,
    description: String,
    url: String,
    timestamp: String,
    footer: EmbedFooter,
}

#[derive(Serialize)]
struct DiscordPayload {
    username: String,
    avatar_url: String,
    embeds: Vec<Embed>,
}
async fn post_to_discord(config: &config::Config, item: &Item, dt: &DateTime<Utc>) {
    let client = reqwest::Client::new();
    client
        .execute(
            client
                .post(config.discord.webhook_url.clone())
                .json(&DiscordPayload {
                    username: config.discord.author_name.clone(),
                    avatar_url: config.discord.avatar_url.clone(),
                    embeds: vec![Embed {
                        r#type: String::from("rich"),
                        title: item.title.clone().unwrap(),
                        description: item.link.clone().unwrap(),
                        url: item.guid.clone().unwrap().value,
                        timestamp: dt.to_rfc3339(),
                        footer: EmbedFooter {
                            text: item.extensions["nyaa"]["category"][0]
                                .value
                                .clone()
                                .unwrap(),
                        },
                    }],
                })
                .build()
                .unwrap(),
        )
        .await
        .unwrap();
}
