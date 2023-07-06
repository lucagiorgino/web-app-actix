use std::{env, sync::{Mutex, RwLock, Arc}, path::PathBuf, fmt::format};
use actix_web::{web, App, HttpServer, middleware::Logger};
use server::{controllers::{expense_controller, income_controller}, AppState};
use mongodb::Client;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let address = env::var("ADDR").expect("$ADDR must be set.");
    let port = env::var("PORT").expect("$PORT must be set.").parse::<u16>().unwrap();

    let usr = env::var("MONGO_INITDB_ROOT_USERNAME").expect("$MONGO_INITDB_ROOT_USERNAME must be set.");
    let pass = env::var("MONGO_INITDB_ROOT_PASSWORD").expect("$MONGO_INITDB_ROOT_PASSWORD must be set.");

    log::info!("Starting up on {}:{}", address, port);

    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| format!("mongodb://{}:{}@localhost:27017", usr, pass));
    let mongo_client = Client::with_uri_str(mongo_uri).await.expect("failed to connect");
    //TODO: create an init function if the collections don't exist
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                AppState {
                    app_name: "actix-demo".to_string()
                })
            )
            .app_data(web::Data::new(mongo_client.clone()))
            .service(web::scope("/api")
                .configure(expense_controller::scoped_config)
                // .configure(income_controller::scoped_config)
            )
            .wrap(Logger::default())
    })
    .bind((address, port))?
    .run()
    .await.map_err(anyhow::Error::from)
}