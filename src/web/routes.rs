use axum::{debug_handler, http::header, response::{IntoResponse, Redirect}, routing::get, Router};

use crate::state::AppState;

use super::views::{self, HtmxContext};



#[debug_handler]
async fn home_view(context: HtmxContext) -> impl IntoResponse {
    views::index::IndexView{context}
}

#[debug_handler]
async fn about_view(context: HtmxContext) -> impl IntoResponse {
    views::index::AboutView{context}
}

#[debug_handler]
async fn style_css() -> impl IntoResponse {
    let style_css = include_str!("../../static/style.css");
    ([(header::CONTENT_TYPE, "text/css")], style_css)
}

async fn index() -> impl IntoResponse {
    Redirect::to("/home")
}

pub fn setup_view_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/home", get(home_view))
        .route("/about", get(about_view))
        .route("/static/style.css", get(style_css))
}
