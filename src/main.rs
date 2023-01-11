use mongodb::{Client, options::{ ClientOptions, ResolverConfig }};
use std::env;
use std::error::Error;
use tokio;
use chrono::{ TimeZone, Utc };
use mongodb::bson::{doc, Bson};
use bson::Document;
use serde::{Serialize, Deserialize};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_uri = "mongodb://localhost:27017";
    
    let options = ClientOptions::parse_with_resolver_config(&db_uri,ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;

    println!("Databases : ");

    for name in client.list_database_names(None, None).await? {
        println!(" - {}", name);
    }

    let apple_fruit = client.database("fruits").collection("apples");


    let new_apple_fruit = doc! {
        "name": "Ooty Apple",
        "rate": 90,
        "rupees": "INR",
        "expiry": Utc.ymd(2023, 01, 31).and_hms(0,0,0),
    };

    let insert_new_apple = apple_fruit.insert_one(new_apple_fruit.clone(), None).await?;

    println!(" New Document ID : {}", insert_new_apple.inserted_id);

    let find_fruit: Document = apple_fruit.find_one(
        doc!{
            "name": "Kashmir Apple"
        },
        None
    ).await?.expect("Missing document");

    println!("Apple find fruit {}", find_fruit);


    let update_fruit = apple_fruit.update_one(
        doc!{
            "_id": &find_fruit.get("_id")
        },
        doc!{
            "$set": { "rate": 140 }
        },
        None
    ).await?;

    println!("Updated document {:?}", update_fruit);




    let delete_fruit = apple_fruit.delete_one(
        doc!{
            "name": "Kashmir Apple"
        },
        None,
    ).await?;

    println!("Deleted {} documents", delete_fruit.deleted_count);

    #[derive(Serialize, Deserialize, Debug)]
    struct Fruit {
        #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
        id: Option<bson::oid::ObjectId>,
        name: String,
        rate: i32,
        rupees: String,
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        expiry: chrono::DateTime<Utc>
    }

    let new_another_apple = Fruit {
        id: None,
        name: "Tamil Nadu Apple".to_owned(),
        rate: 100,
        rupees: "INR".to_owned(),
        expiry: Utc.ymd(2023, 01, 31).and_hms(0,0,0),
    };

    let serialized_apple_fruit = bson::to_bson(&new_another_apple)?;

    let document = serialized_apple_fruit.as_document().unwrap();

    let lets_insert = apple_fruit.insert_one(document.to_owned(), None).await?;

    let inserted_new_apple_id = lets_insert.inserted_id.as_object_id().expect("Failed to get Obejct ID");

    println!("New apple inserted Object ID: {:?}", inserted_new_apple_id);

    //retrieve the uploaded document
    let fetch_apple = apple_fruit.find_one(
        Some(doc!{
            "_id": inserted_new_apple_id.clone()
        }),
        None
    ).await?.expect("Document not found!");

    let fetched_apple_struct: Fruit = bson::from_bson(Bson::Document(fetch_apple))?;

    println!("The fetched apple is {:?}", fetched_apple_struct);



    Ok(())


}
