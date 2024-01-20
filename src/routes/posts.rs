use crate::AppState;
use axum::extract::State;
use axum::response::{Form, Html, IntoResponse, Redirect};
use axum::Extension;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::prelude::*;
use serde::Deserialize;
use std::sync::Arc;
use tera::Context;

use crate::schema::posts;
use crate::utils::db::{get_connection, Pool};

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct NewPostForm {
    title: String,
    body: String,
    published: bool,
}

impl NewPostForm {
    pub fn insert(
        &self,
        conn: &mut PgConnection,
    ) -> Result<usize, diesel::result::Error> {
        insert_into(posts::table).values(self).execute(conn)
    }
}

pub async fn new(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("name", "World");

    let home = state.tera.render("posts/new.html", &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    context.insert("with_footer", &true);

    let rendered = state.tera.render("views/main.html", &context).unwrap();
    Html(rendered)
}

pub async fn create(
    Extension(pool): Extension<Pool>,
    Form(form_data): Form<NewPostForm>,
) -> impl IntoResponse {
    use crate::schema::posts::dsl::*;

    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    match insert_into(posts::table())
        .values(&form_data)
        .execute(&mut conn)
    {
        Ok(_) => Ok(Redirect::to("/")),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
