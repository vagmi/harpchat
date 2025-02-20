use crate::model::{Conversation, Message};
use crate::error::Result;
use genai::chat::{ChatMessage, ChatRequest, ChatRole, MessageContent};
use genai::Client;
use sqlx::PgPool;

impl Into<ChatMessage> for Message {
    fn into(self) -> ChatMessage {
        let role = if self.role == "User" {
            ChatRole::User
        } else {
            ChatRole::Assistant
        };
        ChatMessage { role, content: MessageContent::from_text(self.body)}
    }
}

impl Conversation {
    pub async fn to_chat_request(&self, pool: PgPool) -> Result<ChatRequest> {
        let system_prompt = "Keep the answers direct and be helpful. Keep the language to a graduate level. Ensure that you respond in valid markdown";
        let messages = self.get_messages(pool.clone()).await?;
        let mut chat_req = ChatRequest::default().append_message(ChatMessage::system(system_prompt));
        for msg in messages {
            chat_req = chat_req.append_message(msg.clone());
        }
        Ok(chat_req)
    }
    pub async fn summarize_request(&self, pool: PgPool) -> Result<()> {
        let system_prompt = "You are great at suggesting titles for given conversations.";
        let messages = self.get_messages(pool.clone()).await?;
        let content = messages.iter().map(|msg| msg.body.clone()).collect::<Vec<String>>().join("\n");
        let mut chat_req = ChatRequest::default().append_message(ChatMessage::system(system_prompt));
        let title_prompt = format!("Summarize this conversation\n---\n{}\n---\nOnly give me the title. Do NOT return anything else", content);
        chat_req = chat_req.append_message(ChatMessage::user(title_prompt));
        let client = Client::default();

        let res = client.exec_chat("gpt-4o-mini", chat_req, None).await?;
        if let Some(content) = res.content {
            if let Some(title) = content.text_into_string() {
                self.update_title(pool.clone(), &title).await?;
            }
        }
        Ok(())
    }
    pub async fn send_to_ai(&self, pool: PgPool) -> Result<()> {
        let client = Client::default();
        let req= self.to_chat_request(pool.clone()).await?;
        let res = client.exec_chat("gpt-4o-mini", req, None).await?;
        if let Some(content) = res.content {
            if let Some(msg_str) = content.text_into_string() {
                self.create_message(pool.clone(), &msg_str, Some("Assistant")).await?;
                self.summarize_request(pool).await?;
            }
        }
        Ok(())
    }
}
