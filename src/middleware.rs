use crate::AppState;
use axum::body::Body;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use std::sync::Arc;
use tera::Context;

pub async fn handle_error(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let response = next.run(req).await;

    let status_code = response.status().as_u16();
    let status_text = response.status().as_str().to_string();

    match status_code {
        _ if status_code >= 400 => {
            let mut context = Context::new();
            context.insert("status_code", &status_code);
            context.insert("status_text", &status_text);

            let error =
                state.tera.render("views/error.html", &context).unwrap();

            let mut context = Context::new();
            context.insert("view", &error);
            context.insert("with_footer", &true);
            let rendered =
                state.tera.render("views/main.html", &context).unwrap();
            let h = Html(rendered).into_response();
            Ok(h)
        }

        _ => Ok(response),
    }
}
