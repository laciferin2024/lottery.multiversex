use multiversx_sc_snippets::imports::*;
use rust_interact::lottery_cli;
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(tracing::Level::DEBUG).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting tracing subscriber failed");

    // tracing_subscriber::fmt::init(); // Initializes logging
    lottery_cli().await;
}  

