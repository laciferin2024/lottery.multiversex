use crate::{proxy, ContractInteract, State};
use multiversx_sc::imports::{BigUint, ReturnsResultUnmanaged, TokenIdentifier};
use multiversx_sc_snippets::imports::{Bech32Address, StaticApi};
use multiversx_sc_snippets::InteractorRunAsync;

impl ContractInteract {
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
            .payment((
                TokenIdentifier::from(token_id.as_str()),
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

    pub async fn token_id(&mut self) -> String {
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
        result_value.to_string()
    }
}
