use maud::{html, Markup, PreEscaped, Render};

use crate::model::{Conversation, Message};

use super::{icons::send_icon, layout, HtmxContext};

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
        if self.id == 0 {
            return html! {
                div ."sse-container" {
                    (self)
                }
            };
        }
        html! {
            (self)
            div ."sse-container" {}
        }
    }
}
static SPINNER: &str = r#"
<svg class="text-gray-300 animate-spin" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg"
    width="24" height="24">
    <path
      d="M32 3C35.8083 3 39.5794 3.75011 43.0978 5.20749C46.6163 6.66488 49.8132 8.80101 52.5061 11.4939C55.199 14.1868 57.3351 17.3837 58.7925 20.9022C60.2499 24.4206 61 28.1917 61 32C61 35.8083 60.2499 39.5794 58.7925 43.0978C57.3351 46.6163 55.199 49.8132 52.5061 52.5061C49.8132 55.199 46.6163 57.3351 43.0978 58.7925C39.5794 60.2499 35.8083 61 32 61C28.1917 61 24.4206 60.2499 20.9022 58.7925C17.3837 57.3351 14.1868 55.199 11.4939 52.5061C8.801 49.8132 6.66487 46.6163 5.20749 43.0978C3.7501 39.5794 3 35.8083 3 32C3 28.1917 3.75011 24.4206 5.2075 20.9022C6.66489 17.3837 8.80101 14.1868 11.4939 11.4939C14.1868 8.80099 17.3838 6.66487 20.9022 5.20749C24.4206 3.7501 28.1917 3 32 3L32 3Z"
      stroke="currentColor" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"></path>
    <path
      d="M32 3C36.5778 3 41.0906 4.08374 45.1692 6.16256C49.2477 8.24138 52.7762 11.2562 55.466 14.9605C58.1558 18.6647 59.9304 22.9531 60.6448 27.4748C61.3591 31.9965 60.9928 36.6232 59.5759 40.9762"
      stroke="currentColor" stroke-width="5" stroke-linecap="round" stroke-linejoin="round" class="text-gray-900">
    </path>
  </svg>
"#;

pub fn message_form(conversation: &Conversation) -> Markup {
    let post_uri = format!("/conversations/{}", conversation.id);
    html! {
        form #message-form ."flex flex-row" method="post" action=(post_uri) {
            div ."flex-1 border-1 border-violet-300" {
                input ."w-full p-2 focus:outline-violet-700" type="text" name="message" placeholder="Ask me anything" required="true";
            }
            div #"loading" ."htmx-indicator" {
                (PreEscaped(SPINNER))
            }
            div ."my-2 ml-2 cursor-pointer text-violet-700" {
                button ."cursor-pointer" hx-indicator="#loading" hx-swap="outerHTML" hx-target="#message-form" hx-post=(post_uri) hx-swap="none" type="submit" { (send_icon()) }
            }
        }
    }
}

impl Render for ConversationDetail {
    fn render(&self) -> maud::Markup {
        let body = html! {
            div ."flex flex-col flex-1 h-full" {
                h1 ."text-xl" { (self.conversation.title) }
                div hx-ext="sse" sse-connect={"/conversations/" (self.conversation.id) "/subscribe" } hx-swap="outerHTML" sse-swap="chat" hx-target=".sse-container" ."flex-1 overflow-auto" {
                    @for message in &self.messages {
                        (message)
                    }
                    div ."sse-container" {}
                }
                {(message_form(&self.conversation))}
            }
        };
        body
    }
}
