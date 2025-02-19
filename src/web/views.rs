use axum::{
    extract::{FromRequestParts, OriginalUri},
    http::{request::Parts, Uri},
};
use maud::{html, Markup};

pub mod icons;
pub mod index;
pub mod conversations;

#[derive(Debug, Clone)]
pub struct HtmxContext {
    pub uri: Uri,
    pub is_hx_req: bool,
    pub is_boost: bool,
}

impl HtmxContext {
    pub fn is_partial(&self) -> bool {
        self.is_hx_req && !self.is_boost
    }
}

impl<S> FromRequestParts<S> for HtmxContext
where
    S: Send + Sync,
{
    type Rejection = crate::error::HarpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let OriginalUri(uri) = OriginalUri::from_request_parts(parts, state).await.unwrap();
        let is_hx_req = if parts.headers.get("Hx-Request") == None {
            false
        } else {
            true
        };
        if is_hx_req {
            let is_boost = if parts.headers.get("Hx-Boosted") == None {
                false
            } else {
                true
            };
            return Ok(Self {
                uri,
                is_hx_req,
                is_boost,
            });
        }
        Ok(Self {
            uri,
            is_hx_req,
            is_boost: false,
        })
    }
}

fn nav(current_path: &str) -> Markup {
    let paths = vec![("/conversations", "Conversations"), ("/about", "About")];
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
pub fn not_found(context: HtmxContext) -> Markup {
    html! {
        h1 { "404 Not Found" }
        p { "The page you are looking for does not exist." }
        p { (context.uri) }
    }
}

fn layout(title: Option<String>, body: Markup, context: HtmxContext) -> Markup {
    if context.is_partial() {
        return body;
    }
    let title = title.unwrap_or_else(|| "Harp Chat".to_string());
    html! {
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/static/style.css";
                script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {}
            }
            body ."min-h-screen flex flex-col" {
                div ."container mx-auto" {
                    (nav(&context.uri.to_string()));
                }

                div ."app flex-1 container mx-auto overflow-y-auto pb-20" {
                    (body)
                }
                
                footer ."fixed bottom-0 w-full bg-white border-t py-4" {
                    div ."container mx-auto" {
                        a href="https://github.com/vagmi/harpchat" target="_blank" { "github.com/vagmi/harpchat" }
                    }
                }
            }
        }
    }
}
