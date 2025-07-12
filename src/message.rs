type NatsMessage = async_nats::Message;

#[derive(Debug, Clone)]
pub struct Message {
    pub subject: String,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(subject: String, payload: Vec<u8>) -> Self {
        Message { subject, payload }
    }
}

impl From<NatsMessage> for Message {
    fn from(message: NatsMessage) -> Self {
        Message::new(message.subject.into_string(), message.payload.into())
    }
}

impl From<Message> for String {
    fn from(message: Message) -> String {
        String::from_utf8(message.payload).unwrap()
    }
}
