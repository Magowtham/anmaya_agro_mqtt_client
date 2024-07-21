use rust_socketio::{asynchronous::{Client,ClientBuilder},Payload};
use rust_socketio::error::Error as SocketError;

use crate::config::SocketSettings;



pub struct SocketService {
    pub socket:Client
}

impl SocketService {
   pub async fn init(config_settings:&SocketSettings) ->Result<SocketService,SocketError>{
        let socket=ClientBuilder::new(config_settings.uri.clone()).namespace("/").connect().await?;
        println!("connected to socket server");
        return Ok(Self {
            socket,
        });
    }
}