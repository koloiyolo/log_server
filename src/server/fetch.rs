use std::net::UdpSocket;

pub struct FetchServer {
    socket: UdpSocket,
    publisher: async_nats::Client,
    subject: String,
    buffer_size: usize,
}

impl FetchServer {
    pub async fn new(
        server_address: &String,
        queue_address: &String,
        subject: &String,
        buffer_size: Option<usize>,
    ) -> Result<FetchServer, async_nats::Error> {
        let socket = UdpSocket::bind(server_address)?;

        let publisher = async_nats::connect(queue_address).await?;

        let subject = subject.to_string();

        let buffer_size = buffer_size.unwrap_or(4096);

        Ok(FetchServer {
            socket,
            publisher,
            subject,
            buffer_size,
        })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        let mut buffer = vec![0; self.buffer_size];
        let subject = &self.subject[..];
        let publisher = self.publisher;
        loop {
            if let Ok(bytes_read) = self.socket.recv(&mut buffer) {
                let buffer = buffer[..bytes_read].to_vec();
                publisher
                    .publish(subject.to_string(), buffer.into())
                    .await
                    .unwrap_or_else(|e| eprintln!("{e}"))
            }
        }
    }
}
