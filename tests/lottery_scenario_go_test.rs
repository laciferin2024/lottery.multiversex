use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn lottery_go() {
    world().run("scenarios/lottery.scen.json");
}
