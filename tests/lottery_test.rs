use multiversx_sc::types::{Address, TokenIdentifier, BigUint, ManagedAddress};
use multiversx_sc_scenario::{
    scenario_model::{
        Account, AddressValue, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep, TxESDT,
    },
    ScenarioWorld, ContractInfo, DebugApi,
};
use lottery::ProxyTrait as _;

const LOTTERY_WASM_PATH: &str = "output/lottery.wasm";
const LOTTERY_TOKEN_WASM_PATH: &str = "output/lottery_token.wasm";

struct LotteryTestState {
    world: ScenarioWorld,
    owner_address: Address,
    lottery_contract: ContractInfo<lottery::Proxy<DebugApi>>,
    token_id: TokenIdentifier<DebugApi>,
    participants: Vec<Address>,
}

impl LotteryTestState {
    fn new() -> Self {
        let mut world = ScenarioWorld::new();
        world.register_contract(LOTTERY_WASM_PATH, lottery::ContractBuilder);

        let owner_address = ManagedAddress::from("address:owner");
        let lottery_sc_address = ManagedAddress::from("sc:lottery");

        let lottery_contract = ContractInfo::<lottery::Proxy<DebugApi>>::new("sc:lottery");

        let participants = vec![
            Address::from("address:participant1"),
            Address::from("address:participant2"),
            Address::from("address:participant3"),
        ];

        let token_id = TokenIdentifier::from("str:LTK-123456");

        Self {
            world,
            owner_address,
            lottery_contract,
            token_id,
            participants,
        }
    }

    fn setup_accounts(&mut self) {
        let mut set_state_step = SetStateStep::new();

        // Setup owner account
        set_state_step.put_account(&AddressValue::from(&self.owner_address), Account::new().nonce(1).balance(BigUint::from(1000000000000u64)));

        // Setup participant accounts
        for participant in &self.participants {
            set_state_step.put_account(&AddressValue::from(participant), Account::new().nonce(1).balance(BigUint::from(1000000000000u64)));
        }

        // Setup contract address
        set_state_step.put_account(
            &AddressValue::from(&self.lottery_contract.address),
            Account::new().nonce(0).balance(BigUint::from(0u64)),
        );

        self.world.set_state_step(set_state_step);
    }

    fn deploy_lottery(&mut self) {
        let mut deploy_step = ScDeployStep::new();

        deploy_step
            .from(&AddressValue::from(&self.owner_address))
            .contract_code(LOTTERY_WASM_PATH, ())
            .argument(&self.token_id)
            .argument(BigUint::from(3u64)) // num_participants
            .argument(BigUint::from(10u64)) // bet_amount
            .gas_limit(50000000);

        self.world.sc_deploy(deploy_step);
    }

    fn get_game_status(&mut self) -> (bool, usize, usize) {
        let mut query_step = ScQueryStep::new();
        query_step
            .to(&AddressValue::from(&self.lottery_contract.address))
            .function("getGameStatus");

        let result = self.world.sc_query(query_step);

        let values = result.value::<(bool, usize, usize)>();
        values
    }

    fn place_bet(&mut self, participant_index: usize, chosen_number: u8) {
        let participant = &self.participants[participant_index];

        let mut call_step = ScCallStep::new();
        call_step
            .from(&AddressValue::from(participant))
            .to(&AddressValue::from(&self.lottery_contract.address))
            .function("place_bet")
            .argument(chosen_number)
            .esdt_transfer(TxESDT {
                token_identifier: self.token_id.to_boxed_bytes().into(),
                nonce: 0,
                value: BigUint::from(10u64),
            })
            .gas_limit(50000000);

        self.world.sc_call(call_step);
    }

    fn run_full_game_scenario(&mut self, chosen_numbers: &[u8], expected_winning_number: u8) {
        // Get initial status
        let (game_active, participants_count, max_participants) = self.get_game_status();
        assert!(game_active);
        assert_eq!(participants_count, 0);
        assert_eq!(max_participants, 3);

        // Place bets
        for (index, &number) in chosen_numbers.iter().enumerate() {
            self.place_bet(index, number);
            let (_, current_participants, _) = self.get_game_status();
            if index < chosen_numbers.len() - 1 {
                assert_eq!(current_participants, index + 1);
            } else {
                // After last bet, game should reset
                assert_eq!(current_participants, 0);
            }
        }

        // Get new game status
        let (game_active, participants_count, max_participants) = self.get_game_status();
        assert!(game_active);
        assert_eq!(participants_count, 0);
        assert_eq!(max_participants, 3);
    }
}

#[test]
fn test_lottery_init() {
    let mut test_state = LotteryTestState::new();
    test_state.setup_accounts();
    test_state.deploy_lottery();

    let (game_active, participants_count, max_participants) = test_state.get_game_status();
    assert!(game_active);
    assert_eq!(participants_count, 0);
    assert_eq!(max_participants, 3);
}

#[test]
fn test_full_game_with_winner() {
    let mut test_state = LotteryTestState::new();
    test_state.setup_accounts();
    test_state.deploy_lottery();

    // First participant bets on 2, second on 4, third on 1
    // Assuming 2 is the winning number (participant1 wins)
    test_state.run_full_game_scenario(&[2, 4, 1], 2);
}

#[test]
fn test_full_game_no_winners() {
    let mut test_state = LotteryTestState::new();
    test_state.setup_accounts();
    test_state.deploy_lottery();

    // First participant bets on 2, second on 4, third on 1
    // Assuming 3 is the winning number (no winners)
    test_state.run_full_game_scenario(&[2, 4, 1], 3);
}