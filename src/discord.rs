

//https://discord.com/api/webhooks/1281546208275337226/mQdqvaM_ku79O95lyFTunPDIlubfwKB3takXBP6EmbiUtZk96OTOETGFRBxdBSw9XVVc

use std::error::Error;
use std::future::Future;
use futures::future::FutureExt;
use serde::Serialize;
use reqwest::Client;
#[derive(Serialize)]
pub struct DiscordMessage {
    content: String,
    username: Option<String>, // Optional: Can set a custom username
    avatar_url: Option<String>, // Optional: Can set a custom avatar URL
}
impl DiscordMessage {
    pub fn new(content:String) -> DiscordMessage {
        DiscordMessage{content, username: None, avatar_url: None}
    }
}
#[derive(Debug)]
pub struct DiscordClient {
    client: Client,
    webhook:String
}

impl DiscordClient {
    pub fn new() -> Self {
        let webhook = "https://discord.com/api/webhooks/1281546208275337226/mQdqvaM_ku79O95lyFTunPDIlubfwKB3takXBP6EmbiUtZk96OTOETGFRBxdBSw9XVVc".to_string();

        // Create the HTTP client
        let client = Client::new();
        DiscordClient{client, webhook}
    }
    pub fn send(&self, msg:DiscordMessage) -> impl Future<Output =()> {
        self.client.post(&self.webhook).json(&msg).send().map(|_| ())
    }
}

