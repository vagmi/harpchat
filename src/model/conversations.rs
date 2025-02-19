use chrono::NaiveDateTime;
use sqlx::{postgres::PgPoolCopyExt, PgPool};
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Conversation {
    pub async fn create(pool: PgPool, title: &str) -> Result<Conversation> {
        let rec = sqlx::query_as!(
            Conversation,
            r#"INSERT INTO conversations (title) VALUES ($1) RETURNING *"#,
            title
        )
        .fetch_one(&pool)
        .await?;
        Ok(rec)
    }
    pub async fn get_all(pool: PgPool) -> Result<Vec<Conversation>> {
        let recs = sqlx::query_as!(
            Conversation,
            r#"SELECT * FROM conversations"#
        )
        .fetch_all(&pool)
        .await?;
        Ok(recs)
    }
    pub async fn get_messages(&self, pool: PgPool) -> Result<Vec<Message>> {
        let recs = sqlx::query_as!(
            Message,
            r#"SELECT * FROM messages WHERE conversation_id = $1 order by created_at asc"#,
            self.id
        )
        .fetch_all(&pool)
        .await?;
        Ok(recs)
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub role: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

