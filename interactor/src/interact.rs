#![allow(non_snake_case)]

pub mod config;
mod i;
mod proxy;
mod interact_cli;

use bech32::encode;
use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::{
    io::{Read, Write},
    panic,
    path::Path,
};
use lottery::__wasm__endpoints__::token_id;

const STATE_FILE: &str = "state.toml";

pub async fn lottery_cli() {
    env_logger::init();

    let mut args = RefCell::new(std::env::args().skip(1)); //program name

    let _arg = || -> Option<String> { args.borrow_mut().next() };

    let cmd = _arg().expect("at least one argument required");
    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;

    let arg = || -> String { _arg().expect("expected argument") };

    let mut get_addr = || -> Bech32Address {
        let address = _arg();
        if address.is_some() {
            let address = Bech32Address::from_bech32_string(address.unwrap());

            dbg!("Address from Bech32: {}", address.clone());
            return address;
        }

        // let address = arg();
        let address = interact.wallet_address.clone();
        let address = Bech32Address::from_bech32_string(encode(&address));
        dbg!("Wallet Address: {}", address.clone());
        address
    };

    match cmd.as_str() {
        "deploy" => {
            let num_participants = _arg().unwrap_or("1".to_string()).parse::<usize>().unwrap();
            let token_id_str = _arg().unwrap_or("EGLD".to_string());

            let token_id = if token_id_str == "EGLD" {
                EgldOrEsdtTokenIdentifier::egld()
            } else {
                EgldOrEsdtTokenIdentifier::from(ManagedBuffer::from(token_id_str))
            };
            // let token_id = ManagedBuffer::from(token_id);

            interact.deploy(num_participants, multiversx_sc::imports::OptionalValue::Some(token_id)).await;
        }
        "upgrade" => interact.upgrade().await,
        "place_bet" => {
            let no = arg();
            interact.place_bet(no.parse::<u8>().unwrap()).await
        }
        "getGameStatus" => interact.get_game_status().await,
        "mint" => {
            let address = get_addr();
            interact.mint(address.clone(), 1000u128).await;
            interact.get_token_balance(address).await
        }
        "getTokenBalance" => interact.get_token_balance(get_addr()).await,
        "token_id" => {
            interact.token_id().await;
        }
        "num_participants" => interact.num_participants().await,
        "bet_amount" => {
            interact.bet_amount().await;
        }
        "game_active" => interact.game_active().await,
        "current_game_id" => interact.current_game_id().await,
        "player_numbers" => interact.player_numbers().await,
        "has_placed_bet" => interact.has_placed_bet().await,
        "burn" => interact.burn().await,
        "transfer" => interact.transfer().await,
        "getTokenSupply" => interact.get_token_supply().await,
        "add_liquidity_egld" => interact.add_liquidity_egld().await,
        "remove_liquidity" => interact.remove_liquidity().await,
        "swap_egld_for_tokens" => interact.swap_egld_for_tokens().await,
        "swap_tokens_for_egld" => interact.swap_tokens_for_egld().await,
        "getPoolInfo" => interact.get_pool_info().await,
        "getLpBalance" => interact.get_lp_balance().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_game_status(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .get_game_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn num_participants(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .num_participants()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn game_active(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .game_active()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn current_game_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .current_game_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn player_numbers(&mut self) {
        let game_id = 0u32;
        let player = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .player_numbers(game_id, player)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn has_placed_bet(&mut self) {
        let game_id = 0u32;
        let player = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .has_placed_bet(game_id, player)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn burn(&mut self) {
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .burn(amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn transfer(&mut self) {
        let recipient = bech32::decode("");
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .transfer(recipient, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_token_supply(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .get_token_supply()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn add_liquidity_egld(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let custom_token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .add_liquidity_egld(custom_token_amount)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn remove_liquidity(&mut self) {
        let lp_token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .remove_liquidity(lp_token_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn swap_egld_for_tokens(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .swap_egld_for_tokens()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn swap_tokens_for_egld(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .swap_tokens_for_egld()
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

    pub async fn get_pool_info(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .get_pool_info()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_lp_balance(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .get_lp_balance(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
