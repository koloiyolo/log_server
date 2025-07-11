use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Default address of the server
    #[arg(short, long, default_value_t=String::from("0.0.0.0"))]
    pub address: String,

    /// Log collection port
    #[arg(short, long, default_value_t = 5014)]
    pub fetch_port: u16,

    /// Main port of API server
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,

    /// Your [nats.io](https://nats.io) connection string. Be carefull,
    ///
    /// by default it uses nats demo server where Your data is unsafe.
    #[arg(short, long, default_value_t = String::from("demo.nats.io"))]
    pub nats_address: String,

    /// Subject for nats subscriber to subscribe.
    #[arg(short, long, default_value_t = String::from("test_subject348573485789345789"))]
    pub subject: String,
}
