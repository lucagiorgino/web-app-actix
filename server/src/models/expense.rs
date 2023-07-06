use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson, Document};

// TODO: models and dto may overlap
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub date: String,
    pub amount: f32,
    pub account: String,
    pub first_category: String,
    pub second_category: String,
    pub notes: String
}


// impl From<Expense> for Bson {
//     fn from(expense: Expense) -> Self {
//         let mut document = Document::new();
//         document.insert("date", expense.date);
//         // ...
//         Bson::Document(document)
//     }
// }