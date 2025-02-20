use std::{convert::Infallible, time::Duration};

use axum::{
    debug_handler,
    
    extract::{Path, State},
    http::{header, HeaderMap},
    response::{IntoResponse, Redirect, sse::{Sse, Event, KeepAlive}},
    routing::{get, post},
    Form, Router,
};
use chrono::format::Item;
use maud::{html, Markup, Render};
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream, StreamExt};

use crate::{
    error::Result,
    model::{Conversation, Message},
    state::AppState,
};

use super::views::{
    self, conversation_detail::ConversationDetail, conversations, not_found, HtmxContext,
};

#[debug_handler]
async fn conversations_view(
    State(app_state): State<AppState>,
    context: HtmxContext,
) -> Result<Markup> {
    let conversations = Conversation::get_all(app_state.db_pool).await?;
    Ok(conversations::ConversationsIndex::new(context, conversations).render())
}

#[debug_handler]
async fn new_conversation(
    State(app_state): State<AppState>,
    context: HtmxContext,
) -> Result<(HeaderMap, Markup)> {
    let pool = app_state.db_pool;
    let conversation = Conversation::create(pool.clone(), "New Conversation").await?;
    let conversations = Conversation::get_all(pool.clone()).await?;
    let messages = conversation.get_messages(pool).await?;
    let mut headers = HeaderMap::new();
    headers.insert(
        "HX-Push-Url",
        format!("/conversations/{}", conversation.id)
            .parse()
            .unwrap(),
    );
    return Ok((
        headers,
        conversations::ConversationsIndex::new_with_detail(
            context,
            conversations,
            conversation.clone(),
            messages,
        )
        .render(),
    ));
}

#[debug_handler]
async fn conversation_view(
    State(app_state): State<AppState>,
    context: HtmxContext,
    Path(id): Path<i32>,
) -> Result<Markup> {
    let pool = app_state.db_pool.clone();
    let conversations = Conversation::get_all(pool.clone()).await?;
    let cloned_conv = conversations.clone();
    let conversation = cloned_conv.iter().filter(|c| c.id == id).next();
    if let Some(conversation) = conversation.clone() {
        let messages = conversation.get_messages(pool).await?;
        match context.is_partial() {
            true => {
                return Ok(ConversationDetail {
                    context,
                    conversation: conversation.clone(),
                    messages: messages.clone(),
                }
                .render())
            }
            false => {
                return Ok(conversations::ConversationsIndex::new_with_detail(
                    context,
                    conversations,
                    conversation.clone(),
                    messages,
                )
                .render())
            }
        }
    } else {
        return Ok(not_found(context));
    }
}

#[derive(serde::Deserialize)]
pub struct MessageForm {
    pub message: String,
}

#[debug_handler]
async fn send_message(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(msg_form): Form<MessageForm>,
) -> Result<()> {
    let msg = msg_form.message;
    let pool = state.db_pool;
    let conversation = Conversation::find(pool.clone(), id).await?;
    let _messages = conversation.create_message(pool.clone(), &msg, None).await?;
    conversation.send_to_ai(pool.clone()).await?;
    Ok(())
}
#[debug_handler]
async fn subscribe_handler(
    State(app_state): State<AppState>,
    context: HtmxContext,
    Path(id): Path<i32>,
) -> Sse<impl Stream<Item=std::result::Result<Event, Infallible>>> {

    let pool = app_state.db_pool.clone();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
    tokio::spawn(Message::subscribe(id, pool, tx));
    let stream = UnboundedReceiverStream::new(rx).map(|f| {
        Ok(Event::default().data(f.render_with_sse().into_string()).event("chat"))
    });

    let sse = Sse::new(stream);

    sse
}

#[debug_handler]
async fn about_view(context: HtmxContext) -> impl IntoResponse {
    views::index::AboutView { context }
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
        .route("/conversations/{id}/subscribe", get(subscribe_handler))
        .route("/about", get(about_view))
        .route("/static/style.css", get(style_css))
}
