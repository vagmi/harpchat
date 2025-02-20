use crate::{error::Result, model::Message};
use chrono::NaiveDateTime;
use sqlx::PgPool;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: i32,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Conversation {
    pub async fn find(pool: PgPool, id: i32) -> Result<Conversation> {
        let rec = sqlx::query_as!(Conversation, r#"SELECT * FROM conversations WHERE id = $1"#, id)
            .fetch_one(&pool)
            .await?;
        Ok(rec)
    }
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
    pub async fn update_title(&self, pool: PgPool, title: &str) -> Result<Conversation> {
        let rec = sqlx::query_as!(
            Conversation,
            r#"update conversations set title=$1 where id=$2 returning *"#,
            title, self.id
        )
        .fetch_one(&pool)
        .await?;
        Ok(rec)
    }
    pub async fn get_all(pool: PgPool) -> Result<Vec<Conversation>> {
        let recs = sqlx::query_as!(Conversation, r#"SELECT * FROM conversations"#)
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
    pub async fn create_message(&self, pool: PgPool, message: &str, role: Option<&str>) -> Result<Vec<Message>> {
        let role = role.unwrap_or("User");
        let _rec = sqlx::query_as!(
            Message,
            r#"INSERT INTO messages (conversation_id, role, body) VALUES ($1, $2, $3) RETURNING *"#,
            self.id,
            role,
            message
        )
        .fetch_one(&pool)
        .await?;
        self.get_messages(pool).await
    }
}

