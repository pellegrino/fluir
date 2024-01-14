extern crate diesel;

use crate::app::app;
use dotenv::dotenv;

mod app;
mod models;
mod routes;
mod schema;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = utils::db::establish_connection();

    // Build our application with a single route
    let app = app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
