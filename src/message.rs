use poem_openapi::Object;

#[derive(Debug, Clone, Object)]
pub struct Message {
    pub subject: String,
    pub payload: String,
}

impl Message {
    pub fn new(subject: String, payload: Vec<u8>) -> Self {
        let payload = String::from_utf8(payload).unwrap_or_else(|e| panic!("Parsing error {e}"));
        Message { subject, payload }
    }
}

type NatsMessage = async_nats::Message;
impl From<NatsMessage> for Message {
    fn from(message: NatsMessage) -> Self {
        Message::new(message.subject.into_string(), message.payload.into())
    }
}
