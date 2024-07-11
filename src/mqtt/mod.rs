use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, Publish, QoS};
use std::time::Duration;
use std::error::Error;



use crate::config::MqttSettings;
use crate::db::MongodbService;

use crate::models::IOTAck;
use crate::models::RelaysState;

pub struct MqttService {
    pub  client:AsyncClient,
    pub  event_loop:EventLoop
}


impl MqttService {
    pub async fn init(config_settings:&MqttSettings) -> Result<Self,ClientError> {

        let mut mqtt_options=MqttOptions::new(&config_settings.client_id,&config_settings.host, config_settings.port);

        mqtt_options.set_keep_alive(Duration::from_secs(config_settings.keep_alive));

        let (client,event_loop)=AsyncClient::new(mqtt_options,10);

        client.subscribe(&config_settings.subscribe_topic1, QoS::AtLeastOnce).await?;
        client.subscribe(&config_settings.subscribe_topic2, QoS::AtLeastOnce).await?;
        client.subscribe(&config_settings.subscribe_topic3, QoS::AtLeastOnce).await?;

        println!("connected to broker");

        return Ok( Self {
            client,
            event_loop
        });
    }

    async fn unit_logger(db:&MongodbService,unit_id:String,relay_id:i32,relay_state:bool)->Result<(),Box<dyn Error>>{
        db.insert_unit_log(unit_id,relay_id,relay_state).await?;
        return Ok(());
    }
   
    pub async fn message_handler(self:&Self,db:&MongodbService,packete:Publish)->Result<(),Box<dyn Error>>{
        
        let topic=packete.topic.as_str();
        let message=String::from_utf8(packete.payload.to_vec())?;

        match topic {

            "iot/ack" => {
                let iot_ack:IOTAck=serde_json::from_str(&message)?;
                
                Self::unit_logger(&db,iot_ack.unit_id.clone(),iot_ack.relay_id,iot_ack.relay_state).await?;

                db.update_node_state(iot_ack.unit_id.clone(),iot_ack.relay_id,iot_ack.relay_state).await?;

            },
            "relays/state" => {
                let relays_state_data:RelaysState=serde_json::from_str(&message)?;

                let db_result=db.find_unit_id(&relays_state_data.unit_subscribe_id).await?;

                if let Some(doc_id)=db_result {

                    let unit_id=doc_id.to_hex();

                    for relay in relays_state_data.relays {

                        Self::unit_logger(&db, unit_id.clone(), relay.relay_id, relay.relay_state).await?;

                        db.update_node_state(unit_id.clone(), relay.relay_id, relay.relay_state).await?;
                    }
                }
            },  
            "reboot/ack" => {
                ()
            }
            _ =>()
        }

        Ok(())
    }

}