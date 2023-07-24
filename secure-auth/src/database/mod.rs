use std::env;

pub mod model;
use model::OTP;
use mongodb::bson::DateTime;
use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection};

pub struct SecuredAuthDatabase {
    otp: Collection<OTP>,
}

impl SecuredAuthDatabase {
    pub async fn init() -> Self {
        // dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("$MONGODB_URI is not set");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("secureAuth");
        let otp: Collection<OTP> = db.collection("OTP");
        SecuredAuthDatabase { otp }
    }

    pub async fn create_otp(&self, otp: String, email: String) -> InsertOneResult {
        let new_doc = OTP {
            otp,
            email,
            verified: false,
            created_at: DateTime::now(),
            expiration_time: DateTime::from_millis(DateTime::now().timestamp_millis() + 300000),
        };
        // let new_data = MyData { my_field: Default::default(), /* other fields */ };
        let user = self
            .otp
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating OTP");
        user
        // Ok(user)
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
