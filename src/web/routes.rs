use axum::{debug_handler, http::header, response::IntoResponse, routing::get, Router};
use maud::{html, Markup};

use crate::state::AppState;


fn layout(title: Option<String>, body: Markup) -> Markup {
    let title = title.unwrap_or_else(|| "Harp Chat".to_string());
    html! {
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/static/style.css";
                script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {}
            }
            body {
                (body)
            }
        }
    }
}

#[debug_handler]
async fn index_view() -> Markup {
    layout(Some("Harp Chat".to_string()), html! {
        h1 { "Welcome to Harp Chat" }
        p ."text-2xl" { "This is a simple chat application built with Axum." }
    })
}


async fn style_css() -> impl IntoResponse {
    let style_css = include_str!("../../static/style.css");
    ([(header::CONTENT_TYPE, "text/css")], style_css)
}

pub fn setup_view_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index_view))
        .route("/static/style.css", get(style_css))
}
