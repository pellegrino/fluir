#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template; // bring trait in scop
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate {
    posts: Vec<models::Post>,
}

async fn posts_endpoint(db_pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::posts::dsl::*;
    let mut connection = db_pool.get().expect("Can't get db connection from pool");

    let results = posts
        .filter(published.eq(true))
        .limit(10)
        .load::<models::Post>(&mut connection)
        .expect("Error loading posts");

    let template = PostsTemplate { posts: results };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

async fn list_posts(db_pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::posts::dsl::*;
    let mut connection = db_pool.get().expect("Can't get db connection from pool");

    let results = posts
        .filter(published.eq(true))
        .limit(10)
        .load::<models::Post>(&mut connection)
        .expect("Error loading posts");

    format!("Displaying {} posts", results.len())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(list_posts))
            .route("/api/posts", web::get().to(posts_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
