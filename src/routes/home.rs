use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use diesel::prelude::*;

use crate::models::post::Post;
use crate::utils::db::{get_connection, Pool}; // Import your Post model
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

// Function to get latest posts from the database
fn get_latest_posts(conn: &mut PgConnection) -> QueryResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    posts.order(id.desc()).limit(5).load::<Post>(conn)
}

// Home page route handler

pub async fn index(Extension(pool): Extension<Pool>) -> impl IntoResponse {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    let posts = match get_latest_posts(&mut conn) {
        Ok(posts) => posts,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    let template = IndexTemplate { posts };

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
