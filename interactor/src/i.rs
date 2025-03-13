use crate::{proxy, ContractInteract, State};
use multiversx_sc::imports::{BigUint, CodeMetadata, EgldOrEsdtTokenIdentifier, OptionalValue, ReturnsNewAddress, ReturnsResultUnmanaged};
use multiversx_sc_snippets::imports::{bech32, Bech32Address, BytesValue, InterpretableFrom, InterpreterContext, StaticApi, Wallet};
use multiversx_sc_snippets::{Interactor, InteractorRunAsync};
use crate::config::Config;

impl ContractInteract {
    pub async fn new(config: Config, wallet: Option<Wallet>) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        // .with_tracer("trace1.scen.json");

        interactor.set_current_dir_from_workspace("lottery");

        let hiro = Wallet::from_pem_file("../wallets/hiro.pem");

        let wallet = wallet.unwrap_or(hiro.unwrap());

        // let wallet = test_wallets::alice();

        let wallet_address = interactor.register_wallet(wallet).await;

        println!("wallet: {:?}", wallet_address);

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/lottery.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self, num_participants: usize, token_id: OptionalValue<EgldOrEsdtTokenIdentifier<StaticApi>>, bet_amount: OptionalValue<BigUint<StaticApi>>) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(proxy::LotteryProxy)
            .init(num_participants, token_id, bet_amount)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn upgrade(&mut self, num_participants: OptionalValue<usize>, token_id: OptionalValue<EgldOrEsdtTokenIdentifier<StaticApi>>, bet_amount: OptionalValue<BigUint<StaticApi>>) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(proxy::LotteryProxy)
            .upgrade(num_participants, token_id, bet_amount)
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn place_bet(&mut self, chosen_number: u8) {
        let token_id = self.token_id().await;
        let token_nonce = 0u64;

        let token_amount = self.bet_amount().await;

        // let token_amount = BigUint::from(0u64);
        let token_amount = BigUint::<StaticApi>::from(token_amount);

        dbg!("token_amt: {}", token_amount.clone().to_u64());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .place_bet(chosen_number)
            .with_egld_or_single_esdt_transfer((
                token_id,
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mint(&mut self, address: Bech32Address, value: u128) {
        let amount = BigUint::<StaticApi>::from(value);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .mint(address, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_token_balance(&mut self, address: Bech32Address) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .get_token_balance(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn bet_amount(&mut self) -> multiversx_sc::codec::num_bigint::BigUint {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .bet_amount()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");

        result_value
    }

    pub async fn token_id(&mut self) -> EgldOrEsdtTokenIdentifier<StaticApi> {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
        result_value
    }
}
