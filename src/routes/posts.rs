use askama::Template;
use axum::response::{Form, Html, IntoResponse, Redirect};
use axum::Extension;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::posts;
use crate::utils::db::{get_connection, Pool};

#[derive(Template)]
#[template(path = "posts/new.html")]
struct NewPostTemplate {}

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

pub async fn new() -> impl IntoResponse {
    let template = NewPostTemplate {};

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
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
