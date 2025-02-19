use axum::{debug_handler, extract::Path, http::header, response::{IntoResponse, Redirect}, routing::get, Router};
use maud::Render;

use crate::{model::Conversation, state::AppState};

use super::views::{self, conversations, not_found, HtmxContext};



#[debug_handler]
async fn home_view(context: HtmxContext) -> impl IntoResponse {
    let conversations = Conversation::demo().await;
    conversations::ConversationsIndex{context, conversations, selected_conversation: None, messages: None}
}

#[debug_handler]
async fn conversation_view(context: HtmxContext, Path(id): Path<i32>) -> impl IntoResponse {
    let conversations = Conversation::demo().await;
    let cloned_conv = conversations.clone();
    let conversation = cloned_conv.iter().filter(|c| c.id == id).next();
    if let Some(conversation) = conversation.clone() {
        let messages = conversation.get_messages().await;
        match context.is_partial() {
            true => return conversations::ConversationDetail{context, conversation: conversation.clone(), messages: messages.clone()}.render(),
            false => return conversations::ConversationsIndex{context, conversations, selected_conversation: Some(conversation.clone()), messages: Some(messages)}.render(),
        }
    } else {
        return not_found(context);
    }
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
    Redirect::to("/conversations")
}

pub fn setup_view_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/conversations", get(home_view))
        .route("/conversations/{id}", get(conversation_view))
        .route("/about", get(about_view))
        .route("/static/style.css", get(style_css))
}
