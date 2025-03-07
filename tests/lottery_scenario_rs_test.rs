use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/lottery.mxsc.json", lottery::ContractBuilder);
    blockchain
}

#[test]
fn lottery_rs() {
    world().run("scenarios/lottery.scen.json");
}
