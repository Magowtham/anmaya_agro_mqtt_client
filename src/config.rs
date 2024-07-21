use config::{Config,ConfigError,File};
use serde::Deserialize;

#[derive(Clone,Debug,Deserialize)]
pub struct MqttSettings {
    pub host:String,
    pub port:u16,
    pub client_id:String,
    pub subscribe_topic1:String,
    pub subscribe_topic2:String,
    pub subscribe_topic3:String,
    pub keep_alive:u64
}

#[derive(Clone,Debug,Deserialize)]
pub struct DatabaseSettings {
    pub uri:String,
    pub database_name:String,
    pub collection1_name:String,
    pub collection2_name:String,
    pub collection3_name:String
}


#[derive(Clone,Debug,Deserialize)]
pub struct SocketSettings {
    pub uri:String,
    pub event1:String,
    pub event2:String
}

#[derive(Clone,Debug,Deserialize)]
pub struct Settings {
    pub mqtt:MqttSettings,
    pub database:DatabaseSettings,
    pub socket:SocketSettings
}

impl Settings {
    pub fn new() -> Result<Self,ConfigError> {
       
       let config=Config::builder().add_source(File::with_name("config")).build()?;

       let config_settings=config.try_deserialize::<Settings>()?;

        return Ok(config_settings);
    }
}