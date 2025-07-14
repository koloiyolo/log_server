use clap::Parser;
use log_server::api_server::ApiServer;
use log_server::cli::Cli;
use log_server::fetch_server::FetchServer;
use log_server::processing_server::ProcessingServer;
/// echo "this is a test" | nc -u -q 1 localhost 5014
///
/// In rsyslog.conf:
/// *.* action(type="omfwd" target="127.0.0.1" port="5014" protocol="udp")

const DATABASE_URL: &str = "sqlite://message.db";

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let cli = Cli::parse();

    let fetch_address = format!("{}:{}", cli.address, cli.fetch_port);
    let api_address = format!("{}:{}", cli.address, cli.port);
    let queue_address = cli.nats_address;
    let subject = cli.subject;

    let database_url = DATABASE_URL.to_string();

    let fetch_server = FetchServer::new(&fetch_address, &queue_address, &subject, None).await?;
    let processing_server = ProcessingServer::new(&queue_address, &subject, &database_url).await?;
    let api_server = ApiServer::new(&api_address, &database_url).await?;

    let fetch_server = tokio::spawn(fetch_server.serve());
    let processing_server = tokio::spawn(processing_server.serve());
    let api_server = tokio::spawn(api_server.serve());

    let _ = fetch_server.await;
    let _ = processing_server.await;
    let _ = api_server.await;
    Ok(())
}
