use clap::Args;
use multiversx_sc_snippets::imports::RustBigUint;

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct DeployArgs {
    #[arg(short = 'n', long = "num-participants", default_value = "1")]
    pub num_participants: RustBigUint,

    #[arg(short = 't', long = "token-id", default_value = "EGLD")]
    pub token_id: String,
}

// TODO: may be down the road abstract from ping pong