use clap::Parser;
use log_collection::cli::Cli;
use log_collection::fetch_server::FetchServer;
use log_collection::processing_server::ProcessingServer;
/// echo "this is a test" | nc -u -q 1 localhost 5014
///
/// In rsyslog.conf:
/// *.* action(type="omfwd" target="127.0.0.1" port="5014" protocol="udp")

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let cli = Cli::parse();

    let fetch_address = format!("{}:{}", cli.address, cli.fetch_port);
    let queue_address = cli.nats_address;
    let subject = cli.subject;

    let fetch_server = FetchServer::new(&fetch_address, &queue_address, &subject).await?;
    let processing_server = ProcessingServer::new(&queue_address, &subject).await?;

    let fetch_server = tokio::spawn(fetch_server.serve());
    let processing_server = tokio::spawn(processing_server.serve());

    let _ = fetch_server.await;
    let _ = processing_server.await;
    Ok(())
}
