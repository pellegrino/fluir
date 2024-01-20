use crate::middleware::handle_error;
use crate::routes::{admin, blog, home, posts};
use crate::AppState;
use axum::{routing::get, routing::post, Extension, Router};
use diesel::r2d2::Pool;
use std::sync::Arc;
use tower_livereload::LiveReloadLayer;

pub fn app(
    pool: Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    state: Arc<AppState>,
) -> Router {
    Router::new()
        .route("/", get(home::index))
        .route("/blog", get(blog::index))
        .route("/admin", get(admin::index))
        .route("/posts/new", get(posts::new))
        .route("/posts", post(posts::create))
        .with_state(state.clone())
        .layer(Extension(pool))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            handle_error,
        ))
        .layer(LiveReloadLayer::new())
}
