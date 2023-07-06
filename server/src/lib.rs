pub mod controllers;
pub mod dtos;
pub mod services;
pub mod models;

const DB_NAME: &str = "DEMO";
const COLL_NAME: &str = "Users";

// This struct represents the global state
pub struct AppState {
    pub app_name: String
}