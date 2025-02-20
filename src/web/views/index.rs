use axum::response::IntoResponse;
use maud::{html, Render};

use super::{layout, HtmxContext};

#[derive(Debug, Clone)]
pub struct AboutView {
    pub context: HtmxContext,
}

impl Render for AboutView {
    fn render(&self) -> maud::Markup {
        let body = html! {
            h1 { "This is the about page" }
            p ."text-2xl" { "Awesome about content goes here" }
            p { (self.context.uri) }
        };
        layout(Some("Harp About".to_string()), body, self.context.clone())
    }
}

impl IntoResponse for AboutView {
    fn into_response(self) -> axum::response::Response {
        self.render().into_response()
    }
}
