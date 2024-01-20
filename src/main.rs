extern crate diesel;
use crate::app::app;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod middleware;
mod models;
mod routes;
mod schema;
mod utils;

struct AppState {
    tera: Tera,
}
#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama==debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing_subscriber::fmt::init();
    // TODO: think if this should move to AppState instead.
    let pool = utils::db::establish_connection();

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let app_state = AppState { tera };

    // Build our application with a single route
    let app = app(pool, Arc::new(app_state));

    // let adddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
