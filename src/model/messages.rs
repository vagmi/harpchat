
use crate::error::Result;
use chrono::NaiveDateTime;
use serde_json::Value;
use sqlx::{postgres::PgListener, PgPool};
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub model: Option<String>,
    pub role: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Message {
    pub async fn find(pool: PgPool, id: i32) -> Result<Message> {
        let rec = sqlx::query_as!(Message, r#"SELECT * FROM messages WHERE id = $1"#, id)
            .fetch_one(&pool)
            .await?;
        Ok(rec)
    }
    pub async fn subscribe(
        conversation_id: i32,
        pool: PgPool,
        tx: UnboundedSender<Message>,
    ) -> Result<()> {
        let channel = format!("messages:{}", conversation_id);
        let mut listener = PgListener::connect_with(&pool).await?;
        listener.listen(&channel).await?;
        loop {
            let notification = listener.recv().await?;
            let val = serde_json::from_str::<Value>(notification.payload())?;
            if let Some(num) = val["id"].as_i64() {
                debug!("Message ID: {}", num);
                let msg_id = num as i32;
                let msg = Message::find(pool.clone(), msg_id).await?;
                debug!("Message: {:?}", msg);
                match tx.send(msg) {
                    Ok(_) => {continue},
                    Err(_e) => {
                        // the consumer of this loop is dead. Lets break
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
