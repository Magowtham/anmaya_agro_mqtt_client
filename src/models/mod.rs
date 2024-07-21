
use serde::{Serialize,Deserialize};
use chrono::{DateTime,Utc};
use mongodb::bson::oid::ObjectId;

#[derive(Debug,Serialize,Deserialize)]
pub struct Unit{
    #[serde(rename="_id",skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub farm_id:String,
    pub name:String,
    pub subscribe_id:String,
    pub state:bool,
    pub connected_node_count:i32,
    pub power_consumption:i32
}


#[derive(Debug,Serialize,Deserialize)]
pub struct Node {
    #[serde(rename="_id",skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub unit_id:String,
    pub name:String,
    pub relay_id:i32,
    pub state:bool,
    pub power_consumption:String,
    pub usage_time:String,
  
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UnitLog {
    #[serde(rename="_id",skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub unit_id:String,
    pub relay_id:i32,
    pub relay_state:bool,
    pub date_time:DateTime<Utc>
}


#[derive(Debug,Serialize,Deserialize)]
pub struct IOTAck {
        pub unit_subscribe_id:String,
        pub relay_id:i32,
        pub relay_state:bool
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RestartAck {
     pub unit_subscribe_id:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Relay {
        pub relay_id:i32,
        pub relay_state:bool
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RelaysState {
    pub unit_subscribe_id:String,
    pub relays:Vec<Relay>
}


#[derive(Debug,Serialize,Deserialize)]
pub struct ConnectedNodes {
    pub id:String,
    pub name:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LastWillMessage{
    pub unit_id:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UnitConnectedMessage {
    pub unit_id:String
}

impl UnitLog {
    pub fn new(unit_id:String,relay_id:i32,relay_state:bool,date_time:DateTime<Utc>) -> Self {
        return Self {
            id:None,
            unit_id,
            relay_id,
            relay_state,
            date_time
        };
    }
}