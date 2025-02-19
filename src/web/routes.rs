use axum::{debug_handler, extract::{Path, State}, http::{header, HeaderMap}, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router};
use maud::{Markup, Render};

use crate::{model::{Conversation, Message}, state::AppState, error::Result};

use super::views::{self, conversations, not_found, HtmxContext};



#[debug_handler]
async fn conversations_view(State(app_state): State<AppState>, context: HtmxContext) -> Result<Markup> {
    let conversations = Conversation::get_all(app_state.db_pool).await?;
    Ok(conversations::ConversationsIndex::new(context, conversations).render())
}

#[debug_handler]
async fn new_conversation(State(app_state): State<AppState>, context: HtmxContext) -> Result<(HeaderMap, Markup)> {
    let pool = app_state.db_pool;
    let conversation = Conversation::create(pool.clone(), "New Conversation").await?;
    let conversations = Conversation::get_all(pool.clone()).await?;
    let messages = conversation.get_messages(pool).await?;
    let mut headers = HeaderMap::new();
    headers.insert("HX-Push-Url", format!("/conversations/{}", conversation.id).parse().unwrap());
    return Ok((headers, conversations::ConversationsIndex::new_with_detail(
                context, conversations, 
                conversation.clone(), messages).render()))
}

#[debug_handler]
async fn conversation_view(State(app_state): State<AppState>, context: HtmxContext, Path(id): Path<i32>) -> Result<Markup> {
    let pool= app_state.db_pool.clone();
    let conversations = Conversation::get_all(pool.clone()).await?;
    let cloned_conv = conversations.clone();
    let conversation = cloned_conv.iter().filter(|c| c.id == id).next();
    if let Some(conversation) = conversation.clone() {
        let messages = conversation.get_messages(pool).await?;
        match context.is_partial() {
            true => return Ok(conversations::ConversationDetail{context, conversation: conversation.clone(), messages: messages.clone()}.render()),
            false => return Ok(conversations::ConversationsIndex::new_with_detail(
                context, conversations, 
                conversation.clone(), messages).render())
        }
    } else {
        return Ok(not_found(context));
    }
}

#[derive(serde::Deserialize)]
pub struct MessageForm {
    pub message: String
}

#[debug_handler]
async fn send_message(State(state): State<AppState>, context: HtmxContext, Path(id): Path<i32>, Form(msg_form): Form<MessageForm>) -> Result<Markup> {
    let msg = msg_form.message;
    let pool = state.db_pool;
    let conversations = Conversation::get_all(pool.clone()).await?;
    let cloned_conv = conversations.clone();
    let conversation = cloned_conv.iter().filter(|c| c.id == id).next();
    if let Some(conversation) = conversation.clone() {
        let mut messages = conversation.get_messages(pool).await?;
        messages.push(Message {
            id: messages.len() as i32 + 1,
            conversation_id: conversation.id,
            role: "You".to_string(),
            body: msg,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        });
        match context.is_partial() {
            true => return Ok(conversations::ConversationDetail{context, conversation: conversation.clone(), messages: messages.clone()}.render()),
            false => return Ok(conversations::ConversationsIndex::new_with_detail(
                context, conversations, 
                conversation.clone(), messages).render())
        }
    } else {
        return Ok(not_found(context));
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
        .route("/conversations", get(conversations_view))
        .route("/conversations", post(new_conversation))
        .route("/conversations/{id}", get(conversation_view))
        .route("/conversations/{id}", post(send_message))
        .route("/about", get(about_view))
        .route("/static/style.css", get(style_css))
}
