use multiversx_sc_scenario::*;

const LOTTERY_WASM_PATH: &'static str = "output/lottery.wasm"; // Update this to the correct path to your WASM file

#[test]
fn lottery_e2e() {
    let mut sc_scenario = Scenario::new();

    // Deployer, user and some other address
    let deployer = sc_scenario.wallets().new_wallet();
    let user1 = sc_scenario.wallets().new_wallet();
    let user2 = sc_scenario.wallets().new_wallet();
    let user3 = sc_scenario.wallets().new_wallet();

    // 1. World Setup: Deploy the contract.  This simulates the initial deployment to the blockchain.
    let world = World::new();
    let lottery_wrapper = world.deploy_from_file(
        LOTTERY_WASM_PATH,
        deployer.address(),
        BigUint::zero(),
        |deploy| {
            deploy.init(3u64); //initialise with 3 participants
        },
    );

    // Simulate placing bets
    sc_scenario.simulate_block(|block| {
        block.address(&user1.address()).esdt_transfer(
            "LTRY-94ac38",
            1,
            &BigUint::from(10u64),
            &lottery_wrapper.address(),
            |transfer| {
                transfer.place_bet(5u8);
            },
        );
    });

    sc_scenario.simulate_block(|block| {
        block.address(&user2.address()).esdt_transfer(
            "LTRY-94ac38",
            1,
            &BigUint::from(10u64),
            &lottery_wrapper.address(),
            |transfer| {
                transfer.place_bet(3u8);
            },
        );
    });

    sc_scenario.simulate_block(|block| {
        block.address(&user3.address()).esdt_transfer(
            "LTRY-94ac38",
            1,
            &BigUint::from(10u64),
            &lottery_wrapper.address(),
            |transfer| {
                transfer.place_bet(5u8);
            },
        );
    });

    //Assert game state
    sc_scenario.check_vm_value(
        &lottery_wrapper.address(),
        "gameActive",
        "1",
    );
    sc_scenario.check_vm_value(
        &lottery_wrapper.address(),
        "currentGameId",
        "2",
    );
}
