mod discord;

use std::future::Future;
use tonic::{transport::Server, Request, Response, Status};
use tonic::transport::Channel;
use build_event::build_event_sender_server::{BuildEventSender, BuildEventSenderServer};
use build_event::{BuildEvent};
use crate::build_event::build_event_sender_client::BuildEventSenderClient;
use crate::build_event::{BuildFinished, Info};
use crate::build_event::build_event::Event;
use crate::discord::{DiscordClient, DiscordMessage};
use std::option::Option;
use std::time::Duration;
use tokio::time::sleep;

pub mod build_event {
    tonic::include_proto!("sage.buildevent");
}
#[derive(Debug)]
pub struct AntBuildEventSender{
    discord:DiscordClient
}

impl  AntBuildEventSender {
    fn new() -> Self{
        let discord = DiscordClient::new();
        AntBuildEventSender{discord}
    }
}
#[tonic::async_trait]
impl BuildEventSender for AntBuildEventSender{
    async fn send(&self, event:Request<BuildEvent>)->Result<Response<()>, Status> {
        println!("{:?}",event);
        let message = event.into_inner().event.and_then(|e| {match e {
            Event::BuildStarted(_) => {
                None
            }
            Event::BuildFinished(bf) => {
                let info = bf.info.unwrap();
                let name = info.name;
                let description = info.description.unwrap();
                Some(
                    "$name  $description"
                )

            }
            Event::TaskStarted(_) => {
                None
            }
            Event::TaskFinished(_) => {
                None
            }
            Event::TargetStarted(_) => {
                
                None
            }
            Event::TargetFinished(_) => {
                None
            }
            Event::MessageLogged(_) => {
                None
            }
        }});
        self.discord.send(DiscordMessage::new(message.unwrap().to_string())).await;
        Ok(Response::new(()))
    }
}
#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let build_event_sender = AntBuildEventSender::new();
    tokio::spawn(async move {
                     Server::builder()
                         .add_service(BuildEventSenderServer::new(build_event_sender))
                         .serve(addr).await.unwrap();
                 });
    sleep(Duration::from_millis(10)).await;
    let mut client = BuildEventSenderClient::connect("http://[::1]:50051").await?;
    client.send(build_event_request()).await.unwrap();
    Ok(())
}
fn build_event_request() ->Request<BuildEvent> {
    let be = BuildEvent{
        event : Some(Event::BuildFinished(BuildFinished{
            info: Some(Info{
                name:"hello".to_string(),
                description:Some("world".to_string())
            }),
            exception: None,
        }))
    };
    Request::new (be)
}
