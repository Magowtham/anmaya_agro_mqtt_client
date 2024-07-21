use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, Publish, QoS};
use std::time::Duration;
use std::error::Error;
use serde_json::json;



use crate::config::MqttSettings;
use crate::config::SocketSettings;
use crate::db::MongodbService;
use crate::socket::SocketService;

use crate::models::IOTAck;
use crate::models::RestartAck;
use crate::models::RelaysState;
use crate::models::LastWillMessage;
use crate::models::UnitConnectedMessage;


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
        client.subscribe("unit/disconnected", QoS::AtLeastOnce).await?;

        return Ok( Self {
            client,
            event_loop
        });
    }

    async fn unit_logger(db:&MongodbService,unit_id:String,relay_id:i32,relay_state:bool)->Result<(),Box<dyn Error>>{
        db.insert_unit_log(unit_id,relay_id,relay_state).await?;
        return Ok(());
    }
   
    pub async fn message_handler(self:&Self,socket_settings:&SocketSettings,db:&MongodbService,socket_service:&SocketService,packete:Publish)->Result<(),Box<dyn Error>>{
        
        let topic=packete.topic.as_str();
        let message=String::from_utf8(packete.payload.to_vec())?;

        match topic {

            "iot/control-ack" => {
                let iot_ack:IOTAck=serde_json::from_str(&message)?;

                let db_result=db.find_unit_id(&iot_ack.unit_subscribe_id).await?;

                if let Some(doc_id)=db_result {
                    let unit_id=doc_id.to_hex();
                    Self::unit_logger(&db,unit_id.clone(),iot_ack.relay_id,iot_ack.relay_state).await?;
                    db.update_node_state(unit_id.clone(),iot_ack.relay_id,iot_ack.relay_state).await?;

                    let socket_event=socket_settings.event1.clone();
                    let payload=json!({"unit_id":unit_id,"relay_id":iot_ack.relay_id,"relay_state":iot_ack.relay_state});
                

                    socket_service.socket.emit(socket_event, payload).await?;
                }
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
            "unit/restart-ack" => {
               let restart_ack:RestartAck=serde_json::from_str(&message)?;
               let db_result=db.find_unit_id(&restart_ack.unit_subscribe_id).await?;

               if let Some(doc_id)=db_result {

                let unit_id=doc_id.to_hex();
                let socket_event=socket_settings.event2.clone();

                let payload=json!({"unit_id":unit_id,"unit_restart":true});
                
                socket_service.socket.emit(socket_event, payload).await?;
               }
            },
            "unit/connected" => {
                println!("unit connected");
                let unit_connected_message:UnitConnectedMessage=serde_json::from_str(&message)?;
                db.update_unit_state(unit_connected_message.unit_id, true).await?;
            },
            "unit/disconnected" => {
                println!("unit disconnected");
                let last_will_message:LastWillMessage=serde_json::from_str(&message)?;
                db.update_unit_state(last_will_message.unit_id, false).await?;
            }
            _ =>()
        }

        Ok(())
    }

}