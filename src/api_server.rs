use std::net::TcpListener;

pub struct ApiServer {
    tcp_listener: TcpListener,
}

impl ApiServer {
    pub async fn new(address: &String) -> Result<Self, async_nats::Error> {
        let tcp_listener = TcpListener::bind(address).unwrap_or_else(|e| panic!("{e}"));
        Ok(ApiServer { tcp_listener })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        Ok(())
    }
}
