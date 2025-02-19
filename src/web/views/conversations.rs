use axum::response::IntoResponse;
use maud::{html, Render};

use crate::model::{Conversation, Message};

use super::{layout, HtmxContext, icons::send_icon};

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
}

impl Render for ConversationsIndex {
    fn render(&self) -> maud::Markup {
        let body = html! {
            div ."flex flex-row w-full h-full" {
                div ."flex flex-col basis-1/4" {
                    @for conversation in &self.conversations {
                        @let url = format!("/conversations/{}", conversation.id);
                       div ."my-2 p-3 font-large bg-violet-100 rounded-lg" { 
                           a href={(url)} hx-get={(url)} hx-target=".conversation-detail" hx-push-url="true" {
                           (conversation.title) 
                           }
                       } 
                    }
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
            div ."flex-1" {
                h1 ."text-xl" { (self.conversation.title) }
                @for message in &self.messages {
                    div {
                        p { (message.author) }
                        p { (message.body) }
                    }
                }
            }
            form."flex flex-row" method="post" action=(self.context.uri) {
                div ."flex-1 border-1 border-violet-300" {
                    input ."w-full p-2 focus:outline-violet-700" type="text" name="message" placeholder="Ask me anything" required="true";
                }
                div ."my-2 ml-2 cursor-pointer text-violet-700" {
                    button ."cursor-pointer"  hx-post=(self.context.uri) hx-target=".conversation-detail" type="submit" { (send_icon()) }
                }
            }
        };
        body
    }
}
