use std::env;

pub mod model;
use bson::Bson;
use model::OTP;
use mongodb::bson::doc;
use mongodb::bson::DateTime;
use mongodb::results::InsertOneResult;
use mongodb::results::UpdateResult;
use mongodb::{Client, Collection};
use dotenv::dotenv;

pub struct SecuredAuthDatabase {
    otp: Collection<OTP>,
}

impl SecuredAuthDatabase {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("$MONGODB_URI is not set");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("secureAuth");
        let otp: Collection<OTP> = db.collection("OTP");
        println!("Database connectedd successfull");
        SecuredAuthDatabase { otp }
    }

    pub async fn create_otp(
        &self,
        otp: String,
        email: String,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let new_doc = OTP {
            otp,
            email,
            verified: false,
            created_at: DateTime::now(),
            expiration_time: DateTime::from_millis(DateTime::now().timestamp_millis() + 300000),
        };
        // let new_data = MyData { my_field: Default::default(), /* other fields */ };
        let user = self.otp.insert_one(new_doc, None).await;
        // .ok()
        // .expect("Error creating OTP");
        match user {
            Ok(updated_db) => Ok(updated_db),
            Err(e) => Err(e),
        }
        // Ok(user)
    }

    pub async fn get_otp_by_id(&self, id: Bson) -> Result<Option<OTP>, mongodb::error::Error> {
        println!("ID =  {:?}", id);

        let otp_instance = self.otp.find_one(Some(doc! {"_id" :id }), None).await;
        println!("OTP INSTANCE {:?}", otp_instance);
        match otp_instance {
            Ok(updated_db) => Ok(updated_db),
            Err(e) => Err(e),
        }
    }
    pub async fn mark_otp_as_verified(
        &self,
        id: Bson,
    ) -> Result<UpdateResult, mongodb::error::Error> {
        let updated_otp = self
            .otp
            .update_one(doc! {"_id" :id }, doc! {"$set": {"verified" : true} }, None)
            .await?;
        Ok(updated_otp)
    }
}

// pub async fn connect_to_database() -> mongodb::error::Result<(mongodb::Client)> {
//     let uri: String = env::var("MONGODB_URI").expect("$MONGODB_URI is not set");

//     let client = Client::with_uri_str(uri).await.expect("failed to connect");

//     client
//         .database("admin")
//         .run_command(doc! {"ping": 1}, None)
//         .await?;
//     // println!("Pinged your deployment. You successfully connected to MongoDB!");

//     Ok(client)
// }
