use axum::response::IntoResponse;
use maud::{html, Render};

use crate::model::{Conversation, Message};

use super::{layout, HtmxContext};

#[derive(Debug, Clone)]
pub struct ConversationsIndex {
    pub context: HtmxContext,
    pub conversations: Vec<Conversation>,
    pub selected_conversation: Option<Conversation>,
    pub messages: Option<Vec<Message>>,
}

impl Render for ConversationsIndex {
    fn render(&self) -> maud::Markup {
        let body = html! {
            div ."flex flex-row w-full h-full" {
                div ."flex flex-col basis-1/3" {
                    @for conversation in &self.conversations {
                        @let url = format!("/conversations/{}", conversation.id);
                       div { 
                           a href={(url)} hx-get={(url)} hx-target=".conversation-detail" hx-push-url="true" {
                           (conversation.title) 
                           }
                       } 
                    }
                }
                div ."flex flex-col basis-2/3 conversation-detail h-full" {
                    @if let Some(selected_conversation) = &self.selected_conversation {
                        @let conv_detail = ConversationDetail {
                            context: self.context.clone(),
                            conversation: selected_conversation.clone(),
                            messages: self.messages.clone().unwrap_or_default(),
                        };
                        (conv_detail.render());
                    } @else {
                        h1 { "This is the conversations page" }
                        p ."text-2xl" { "Awesome conversations content goes here" }
                        p { (self.context.uri) }
                    }
                }
            }
        };
        layout(Some("Harp Chat".to_string()), body, self.context.clone())
    }
}

impl IntoResponse for ConversationsIndex {
    fn into_response(self) -> axum::response::Response {
        self.render().into_response()
    }
}

#[derive(Debug, Clone)]
pub struct ConversationDetail {
    pub context: HtmxContext,
    pub conversation: Conversation,
    pub messages: Vec<Message>,
}

impl Render for ConversationDetail {
    fn render(&self) -> maud::Markup {
        let body = html! {
            div {
                h1 ."text-xl" { (self.conversation.title) }
                @for message in &self.messages {
                    div {
                        p { (message.author) }
                        p { (message.body) }
                    }
                }
            }
        };
        body
    }
}
