use async_nats::*;
use futures::StreamExt;

pub struct ProcessingServer {
    subscriber: Subscriber,
}

impl ProcessingServer {
    pub async fn new(queue_addres: &String, subject: &String) -> Result<Self, async_nats::Error> {
        let queue = async_nats::connect(queue_addres).await?;
        let subscriber = queue.subscribe(subject.to_string()).await?;

        Ok(ProcessingServer { subscriber })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        let mut subscriber = self.subscriber;
        while let Some(message) = subscriber.next().await {
            println!("{message:?}");
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
