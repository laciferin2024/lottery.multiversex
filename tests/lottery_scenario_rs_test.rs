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


#[test]
fn lottery_init_rs() {
    world().run("scenarios/lottery-init.scen.json");
}


#[test]
fn lottery_place_bet_rs() {
    world().run("scenarios/lottery-place-bet.scen.json");
}

#[test]
fn lottery_place_multiple_bets_rs() {
    world().run("scenarios/lottery-place-multiple-bets.scen.json");
}

// FIXME:test
// #[test]
// fn lottery_complete_game_rs() {
//     world().run("scenarios/lottery-complete-game.scen.json");
// }

#[test]
fn lottery_place_bet_wrong_amount_rs() {
    world().run("scenarios/lottery-place-bet-wrong-amount.scen.json");
}

#[test]
fn lottery_place_bet_twice_rs() {
    world().run("scenarios/lottery-place-bet-twice.scen.json");
}

#[test]
fn lottery_place_bet_invalid_number_rs() {
    world().run("scenarios/lottery-place-bet-invalid-number.scen.json");
}

#[test]
fn lottery_get_game_status_rs() {
    world().run("scenarios/lottery-get-game-status.scen.json");
}
