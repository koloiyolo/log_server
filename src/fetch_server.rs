// use super::server::Server;
use std::net::UdpSocket;

pub struct FetchServer {
    socket: UdpSocket,
    publisher: async_nats::Client,
    subject: String,
}

impl FetchServer {
    pub async fn new(
        server_address: &String,
        queue_address: &String,
        subject: &String,
    ) -> Result<FetchServer, async_nats::Error> {
        let socket = UdpSocket::bind(server_address)?;

        let publisher = async_nats::connect(queue_address).await?;

        let subject = subject.to_string();

        Ok(FetchServer {
            socket,
            publisher,
            subject,
        })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        let mut buffer = [0; 255];
        let subject = &self.subject[..];
        let publisher = self.publisher;
        loop {
            for _ in self.socket.recv(&mut buffer) {
                let buffer = buffer.to_vec();
                let string = String::from_utf8(buffer.clone())?;
                match publisher.publish(subject.to_string(), buffer.into()).await {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        break;
                    }
                }
            }
        }
    }
}
