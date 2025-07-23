use crate::model::message::Message;
use async_nats::Subscriber;
use futures::StreamExt;
use sqlx::SqlitePool;

pub struct ProcessingServer {
    subscriber: Subscriber,
    pool: SqlitePool,
}

impl ProcessingServer {
    pub async fn new(
        queue_addres: &String,
        subject: &String,
        database_url: &String,
    ) -> Result<Self, async_nats::Error> {
        let queue = async_nats::connect(queue_addres).await?;
        let subscriber = queue.subscribe(subject.to_string()).await?;
        let pool = SqlitePool::connect(database_url).await?;

        Ok(ProcessingServer { subscriber, pool })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        let mut subscriber = self.subscriber;

        while let Some(message) = subscriber.next().await {
            if let Some(message) = Message::from_nats(message) {
                let result = sqlx::query_file!(
                    "sql/message/insert.sql",
                    message.date,
                    message.host,
                    message.program,
                    message.message,
                )
                .execute(&self.pool)
                .await;

                if let Err(e) = result {
                    eprintln!("Database error: {}", e);
                }
            }
        }
        Ok(())
    }
}

//  Example message
//  Message {
//      subject: Subject { bytes: b"test_subject348573485789345789" },
//      reply: None,
//      payload: b"{{Message body}}",
//      headers: None, status: None, description: None, length: 249
//  }
