use maud::{html, Markup, PreEscaped, Render};

use crate::model::{Conversation, Message};

use super::{layout, HtmxContext, icons::send_icon};

#[derive(Debug, Clone)]
pub struct ConversationDetail {
    pub context: HtmxContext,
    pub conversation: Conversation,
    pub messages: Vec<Message>,
}

impl Render for Message {
    fn render(&self) -> maud::Markup {
        let content = markdown::to_html(&self.body);
        html! {
            div ."my-2 p-2 bg-violet-50 rounded-lg" {
                p ."font-bold" { (self.role) }
                p ."prose prose-xl" { (PreEscaped(content)) }
            }
        }
    }
}
impl Message {
    pub fn render_with_sse(&self) -> maud::Markup {
        html! {
            (self)
            div ."sse-container" {}
        }
    }
}

impl Render for ConversationDetail {
    fn render(&self) -> maud::Markup {
        let post_uri = format!("/conversations/{}", self.conversation.id);
        let body = html! {
            div ."flex flex-col flex-1 h-full" {
                h1 ."text-xl" { (self.conversation.title) }
                div hx-ext="sse" sse-connect={"/conversations/" (self.conversation.id) "/subscribe" } hx-swap="outerHTML" sse-swap="chat" hx-target=".sse-container" ."flex-1 overflow-auto" {
                    @for message in &self.messages {
                        (message)
                    }
                    div ."sse-container" {}
                }
                form."flex flex-row" method="post" action=(self.context.uri) {
                    div ."flex-1 border-1 border-violet-300" {
                        input ."w-full p-2 focus:outline-violet-700" type="text" name="message" placeholder="Ask me anything" required="true";
                    }
                    div ."my-2 ml-2 cursor-pointer text-violet-700" {
                        button ."cursor-pointer"  hx-post=(post_uri) hx-swap="none" type="submit" { (send_icon()) }
                    }
                }
            }
        };
        body
    }
}
