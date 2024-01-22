extern crate diesel;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_livereload::LiveReloadLayer;

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

    tracing_subscriber::fmt::init();

    let pool = utils::db::establish_connection();

    // Build our application with a single route
    let app = app(pool).layer(LiveReloadLayer::new());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
