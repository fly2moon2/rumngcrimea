// ref.:https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/
//
// connecting mongodb, default no password
// see with MongoDB Conmpass
// export MONGODB_URI='mongodb://localhost:27017/'
// cargo run

use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
// Document found in mongodb database collection
use bson::document::Document;
use std::env;
use std::error::Error;
use tokio;

use chrono::{TimeZone, Utc};
use mongodb::bson::doc;

// struct
// new type idiom
#[derive(Debug)]
struct Sex (String);

#[derive(Debug)]
struct Person {
  name: String,
  sex: Sex,
  //sex: String,
  dob: String,
  data: Vec<u8>,
}

impl Person {
   fn new(name: &str) -> Person {
      Person {
         name: String::from(name),
         //sex: String::from("M"),
         sex: Sex(String::from("F")),
         dob: String::from("dob"),
         data: Vec::new(),
      }
   }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   // connect database collection
   let soldiers = client.database("crimea").collection("soldier");

   // find document
   // Look up one document:
   let soldier: Document = soldiers.find_one(
      doc! {
            "sex": "F"
      },
      None,
   ).await?
   .expect("Can't find the document.");
   println!("solider F: {}", soldier);

   //
   let new_doc = doc! {
    "title": "Parasite",
    "year": 2020,
    "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
    "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
    //"released": Utc.with_ymd_and_hms(2019, 2, 7,0, 0, 0),
    };
    println!("new_doc {}", new_doc);

   // insert new doc to database
   let insert_result = soldiers.insert_one(new_doc.clone(), None).await?;
   println!("New document ID: {}", insert_result.inserted_id);
  
   // insert multiple
   let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
   ];

   // Insert some documents into the "mydb.books" collection.
   //soldiers.insert_many(docs, None).await?;

   // struct to doc
   //let person=Person::new("Arigato");
   let person=Person::new("Frlando");

   let doc_pers1 = doc! {
      "name": person.name,
   };
   
   let rslt1 = soldiers.insert_one(doc_pers1.clone(), None).await?;

   // Update the document:
   let upd_rslt1 = soldiers.update_one(
      doc! {
         //"name": "Arigato"
         "name" : "Frlando"
      },
      doc! {
         //"$set": { "sex": "F" }
         "$set": { "dob":"birth"}
      },
      None,
   ).await?;
   println!("Updated {} document", upd_rslt1.modified_count);

   // delete many
   let del_rslt1 = soldiers.delete_many(
      doc! {
         "title": "The Great Gatsby"
         //$or: [{"title":"1984"},{"title":"The Great Gatsby"}]
      },
      None,
   ).await?;
   println!("Deleted {} documents", del_rslt1.deleted_count);

   Ok(())
}