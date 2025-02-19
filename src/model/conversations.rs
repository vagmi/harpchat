use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Conversation {
    pub async fn demo() -> Vec<Self> {
        let now = chrono::Local::now().naive_local();
        vec![
            Conversation {
                id: 1,
                title: "Getting Started with Rust".to_string(),
                created_at: now - chrono::Duration::days(4),
            },
            Conversation {
                id: 2,
                title: "Building a Web Server".to_string(),
                created_at: now - chrono::Duration::days(3),
            },
            Conversation {
                id: 3,
                title: "Database Design Discussion".to_string(),
                created_at: now - chrono::Duration::days(2),
            },
            Conversation {
                id: 4,
                title: "Code Review: Authentication System".to_string(),
                created_at: now - chrono::Duration::days(1),
            },
            Conversation {
                id: 5,
                title: "API Design Planning".to_string(),
                created_at: now - chrono::Duration::hours(2),
            },
        ]
    }
    pub async fn get_messages(&self) -> Vec<Message> {
        vec![
            Message {
                id: 1,
                conversation_id: self.id,
                author: "Alice".to_string(),
                body: "Hey, I have a question about this topic.".to_string(),
                created_at: self.created_at,
            },
            Message {
                id: 2,
                conversation_id: self.id,
                author: "Bob".to_string(),
                body: "Sure, what would you like to know?".to_string(),
                created_at: self.created_at + chrono::Duration::minutes(5),
            },
            Message {
                id: 3,
                conversation_id: self.id,
                author: "Alice".to_string(),
                body: "I'm trying to understand how this works in practice.".to_string(),
                created_at: self.created_at + chrono::Duration::minutes(8),
            },
            Message {
                id: 4,
                conversation_id: self.id,
                author: "Bob".to_string(),
                body: "Let me explain with an example...".to_string(),
                created_at: self.created_at + chrono::Duration::minutes(10),
            },
            Message {
                id: 5,
                conversation_id: self.id,
                author: "Charlie".to_string(),
                body: "I can also add some insights from my experience.".to_string(),
                created_at: self.created_at + chrono::Duration::minutes(15),
            }
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub author: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

