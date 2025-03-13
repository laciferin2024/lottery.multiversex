use multiversx_sc_snippets::imports::*;
use rust_interact::lottery_cli;
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initializes logging
    lottery_cli().await;
}  

