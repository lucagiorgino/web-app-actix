use std::env;
use anyhow::{Result, Context};
use mongodb::bson::Bson;
use mongodb::{Database, bson::doc};

use crate::models::expense::{Expense, self};
use crate::COLL_NAME;

// TODO: handle failures
pub async fn add_expense(mongo_db: Database) -> Result<()>  {
    
    log::info!("Storing information in db..."); 
    let collection = mongo_db.collection::<Expense>(COLL_NAME); 
    
    // let expense = Expense {  ...  };

    // let result = collection.insert_one(expense, None).await;
    // let _ = match result {
    //     Ok(result) => result,
    //     Err(error) => {
    //         log::info!("{}", error);
    //         return Err(error.into())
    //     }
    // };

    // let filter = doc! {"date": "2023-07-06"};

    // let result = collection.find_one(Some(filter), None).await.unwrap();
    
    // match result {
    //     Some(expense) => {
    //         println!("{:?}", expense)
    //     },
    //     None => println!("No document found"),
    // }

    Ok(())
}
