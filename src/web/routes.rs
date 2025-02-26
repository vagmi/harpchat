use std::convert::Infallible;

use axum::{
    debug_handler,
    
    extract::{Path, State},
    http::{header, HeaderMap},
    response::{IntoResponse, Redirect, sse::{Sse, Event}},
    routing::{get, post},
    Form, Router,
};
use maud::{Markup, Render};
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream, StreamExt};
use tower_sessions::{session::Id, Session};

use crate::{
    error::Result,
    model::{Conversation, Message},
    state::AppState,
};

use super::views::{
    self, conversation_detail::{message_form, ConversationDetail}, conversations, not_found, HtmxContext,
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
async fn conversations_list(
    State(app_state): State<AppState>,
    context: HtmxContext,
) -> Result<Markup> {
    let conversations = Conversation::get_all(app_state.db_pool).await?;
    Ok(conversations::ConversationsIndex::new(context, conversations).conversation_list())
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
    session: Session,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(msg_form): Form<MessageForm>,
) -> Result<Markup> {
    let sess_id = session.id().unwrap_or(Id(0));
    let tx = state.tx_map.lock().unwrap().get(&sess_id).unwrap().clone();
    let msg = msg_form.message;
    let pool = state.db_pool;
    let conversation = Conversation::find(pool.clone(), id).await?;
    let _messages = conversation.create_message(pool.clone(), &msg, None).await?;
    conversation.stream_from_ai(pool.clone(), tx).await?;
    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", "refreshConversations".parse().unwrap());
    Ok(message_form(&conversation))
}
#[debug_handler]
async fn subscribe_handler(
    session: Session,
    State(app_state): State<AppState>,
    // context: HtmxContext,
    Path(id): Path<i32>,
) -> Sse<impl Stream<Item=std::result::Result<Event, Infallible>>> {
    let sess_id = session.id().unwrap_or(Id(0));
    let pool = app_state.db_pool.clone();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
    app_state.tx_map.lock().unwrap().insert(sess_id, tx.clone());
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
        .route("/conversations/_list", get(conversations_list))
        .route("/conversations", post(new_conversation))
        .route("/conversations/{id}", get(conversation_view))
        .route("/conversations/{id}", post(send_message))
        .route("/conversations/{id}/subscribe", get(subscribe_handler))
        .route("/about", get(about_view))
        .route("/static/style.css", get(style_css))
}
