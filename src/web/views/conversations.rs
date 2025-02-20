use axum::response::IntoResponse;
use maud::{html, Markup, Render};

use crate::{model::{Conversation, Message}};

use super::{layout, HtmxContext, conversation_detail::ConversationDetail};

#[derive(Debug, Clone)]
pub struct ConversationsIndex {
    pub context: HtmxContext,
    pub conversations: Vec<Conversation>,
    pub selected_conversation: Option<Conversation>,
    pub messages: Option<Vec<Message>>,
}

impl ConversationsIndex {
    pub fn new(context: HtmxContext, conversations: Vec<Conversation>) -> Self {
        Self {
            context,
            conversations,
            selected_conversation: None,
            messages: None,
        }
    }
    pub fn new_with_detail(
        context: HtmxContext,
        conversations: Vec<Conversation>,
        selected_conversation: Conversation,
        messages: Vec<Message>,
    ) -> Self {
        Self {
            context,
            conversations,
            selected_conversation: Some(selected_conversation),
            messages: Some(messages),
        }
    }

    fn conversation_list(&self) -> Markup {
        let create_button = html! {
            button ."bg-violet-500 text-white p-2 rounded-lg cursor-pointer" 
                   hx-post="/conversations" 
                   hx-target=".app" { 
                       "New Conversation" 
                   }
            
        };
        if self.conversations.is_empty() {
            return html! {
                "Welcome to Harp Chat! No conversations yet."
                (create_button)
            };
        }
        html! {
            @for conversation in &self.conversations {
                @let url = format!("/conversations/{}", conversation.id);
                div ."my-2 p-3 font-large bg-violet-100 rounded-lg" { 
                    a href={(url)} hx-get={(url)} hx-target=".conversation-detail" hx-push-url="true" {
                        (conversation.title) 
                    }
                } 
            }

            (create_button)
        }
    }
}

impl Render for ConversationsIndex {
    fn render(&self) -> maud::Markup {
        let body = html! {
            div ."flex flex-row w-full h-full" {
                div ."flex flex-col basis-1/4" {
                    (self.conversation_list())
                }
                div ."flex flex-col basis-3/4 conversation-detail h-full ml-2" {
                    @if let Some(selected_conversation) = &self.selected_conversation {
                        @let conv_detail = ConversationDetail {
                            context: self.context.clone(),
                            conversation: selected_conversation.clone(),
                            messages: self.messages.clone().unwrap_or_default(),
                        };
                        (conv_detail.render());
                    } @else {
                        div ."flex flex-col h-full w-full justify-center text-center bg-violet-100"  {
                        h1 { "This is the conversations page" }
                        p ."text-2xl" { "Awesome conversations content goes here" }
                        p { (self.context.uri) }
                        }
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
