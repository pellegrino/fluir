use crate::routes::{admin, blog, home};
use axum::{routing::get, Extension, Router};
use diesel::r2d2::Pool;

pub fn app(
    pool: Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
) -> Router {
    Router::new()
        .route("/", get(home::index))
        .route("/blog", get(blog::index))
        .route("/admin", get(admin::index))
        .layer(Extension(pool))
}
