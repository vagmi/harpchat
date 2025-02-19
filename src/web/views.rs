use axum::{extract::{FromRequestParts, OriginalUri}, http::{request::Parts, Uri}};
use maud::{html, Markup};

pub mod index;

#[derive(Debug, Clone)]
pub struct HtmxContext {
    pub uri: Uri,
    pub is_hx_req: bool,
    pub is_boost: bool,
}

impl<S> FromRequestParts<S> for HtmxContext 
where S: Send + Sync
{
    type Rejection = crate::error::HarpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let OriginalUri(uri) = OriginalUri::from_request_parts(parts, state).await.unwrap();
        let is_hx_req = if parts.headers.get("Hx-Request") == None { false } else { true };
        if is_hx_req {
            let is_boost = if parts.headers.get("Hx-Boosted") == None { false } else { true };
            return Ok(Self { uri, is_hx_req, is_boost });
        }
        Ok(Self { uri, is_hx_req, is_boost: false })

    }
}

fn nav(current_path: &str) -> Markup {
    let paths = vec![("/home", "Home"), ("/about", "About")];
    html! {
        nav ."my-2" hx-boost="true" {
            ul {
                @for (path, label) in paths {
                    li ."inline-block mr-3" { a hx-push-url="true" href=(path) class={
                        @if current_path.starts_with(path) { "active" } @else { "" }} {
                            (label)
                        }
                    }
                }
            }
        }
    }
}

fn layout(title: Option<String>, body: Markup, context: HtmxContext) -> Markup {
    let title = title.unwrap_or_else(|| "Harp Chat".to_string());
    html! {
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/static/style.css";
                script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {}
            }
            body {
                div ."container mx-auto" {
                (nav(&context.uri.to_string()));
                (body)
                }
            }
        }
    }
}
