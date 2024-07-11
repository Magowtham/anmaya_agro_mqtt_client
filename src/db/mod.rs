
use mongodb::{Client, Database,Collection};
use mongodb::error::Error as MongoError;
use mongodb::bson::{doc,oid::ObjectId};
use chrono::Utc;


use crate::config::DatabaseSettings;
use crate::models::Unit;
use crate::models::Node;
use crate::models::UnitLog;

pub struct MongodbService {
    db:Database,
    unit_collection:Collection<Unit>,
    node_collection:Collection<Node>,
    unit_log_collection:Collection<UnitLog>,
}

impl MongodbService {
    pub async fn init(config_settings:&DatabaseSettings)->Result<Self,MongoError>{
        let client=Client::with_uri_str(&config_settings.uri).await?;
        
        let database=client.database(&config_settings.database_name);

        let unit_collection=database.collection(&config_settings.collection1_name);
        let node_collection=database.collection(&config_settings.collection2_name);
        let unit_log_collection=database.collection(&config_settings.collection3_name);

        client.database(&config_settings.database_name).run_command(doc! {"ping":1}).await?;
        println!("connected to mongodb");

        return Ok(Self {
            db:database,
            unit_collection,
            node_collection,
            unit_log_collection
        });

    }

    pub async fn insert_unit_log(self:&Self,unit_id:String,relay_id:i32,relay_state:bool)->Result<(),MongoError> {

        let node_filter_query=doc! {
            "unit_id":&unit_id,
            "relay_id":relay_id,
        };

        let db_result=self.node_collection.find_one(node_filter_query).await?;


        if let Some(node) = db_result {
            if node.state != relay_state {

                let device_log=UnitLog::new(
                        unit_id,
                        relay_id,
                        relay_state,
              Utc::now());

                self.unit_log_collection.insert_one(device_log).await?;
            }
        }
        Ok(())
    }

    pub async fn update_node_state(self:&Self,unit_id:String,relay_id:i32,relay_state:bool) -> Result<(),MongoError>{

        let node_filter_query= doc! {
            "unit_id":&unit_id,
            "relay_id":relay_id
        };

        let db_result=self.node_collection.find_one(node_filter_query.clone()).await?;

        if let Some(node) = db_result {
            if node.state != relay_state {
                let update_query = doc! {
                    "$set":{
                        "state":relay_state
                    }
                };

                self.node_collection.update_one(node_filter_query, update_query).await?;
            }
        }
        Ok(())

    }  

    pub async fn find_unit_id(self:&Self,subscribe_id:&String) -> Result<Option<ObjectId>,MongoError> {

        let unit_filter=doc! {
            "subscribe_id":subscribe_id
        };

        let db_result=self.unit_collection.find_one(unit_filter).await?;
        
        if let Some(unit)=db_result {
            return Ok(unit.id);
        } else {
            return Ok(None);
        }
    } 



    
}