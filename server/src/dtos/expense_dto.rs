use serde::{Deserialize, Serialize};

//TODO: enum for category
#[derive(Deserialize, Serialize)]
pub struct ExpenseRequestDTO {
    pub date: String,
    pub amount: f32,
    pub account: String,
    pub first_category: String,
    pub second_category: String,
    pub notes: String
}


#[derive(Deserialize, Serialize)]
pub struct ExpenseResponseDTO {

}
