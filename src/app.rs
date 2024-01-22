use crate::routes::{admin, blog, home, posts};
use axum::{routing::get, routing::post, Extension, Router};
use diesel::r2d2::Pool;
use tower_http::services::ServeDir;

pub fn app(
    pool: Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
) -> Router {
    let static_files = ServeDir::new("assets");

    Router::new()
        .nest_service("/assets", static_files)
        .route("/", get(home::index))
        .route("/blog", get(blog::index))
        .route("/admin", get(admin::index))
        .route("/posts/new", get(posts::new))
        .route("/posts", post(posts::create))
        .layer(Extension(pool))
}
