#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait LotteryToken {

    fn init_token(
        &self,
        initial_supply: BigUint,
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer
    ) {
        let caller = self.blockchain().get_caller();

        // Set token information
        self.token_supply().set(&initial_supply);
        self.token_name().set(&token_name);
        self.token_ticker().set(&token_ticker);

        // Mint initial supply to contract deployer
        self.token_balance(&caller).set(&initial_supply);
    }

    // Mint tokens - only owner can call
    #[only_owner]
    #[endpoint]
    fn mint(&self, recipient: ManagedAddress, amount: BigUint) {
        let current_supply = self.token_supply().get();
        let new_supply = current_supply + &amount;

        self.token_supply().set(&new_supply);
        self.token_balance(&recipient).update(|balance| *balance += &amount);
    }

    // Burn tokens
    #[endpoint]
    fn burn(&self, amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let caller_balance = self.token_balance(&caller).get();

        require!(caller_balance >= amount, "Insufficient balance");

        let new_balance = &caller_balance - &amount;
        self.token_balance(&caller).set(&new_balance);

        let current_supply = self.token_supply().get();
        let new_supply = &current_supply - &amount;
        self.token_supply().set(&new_supply);
    }

    // Transfer tokens
    #[endpoint]
    fn transfer(&self, recipient: ManagedAddress, amount: BigUint) {
        let sender = self.blockchain().get_caller();
        let sender_balance = self.token_balance(&sender).get();

        require!(sender_balance >= amount, "Insufficient balance");

        // Deduct from sender
        self.token_balance(&sender).update(|balance| *balance -= &amount);

        // Add to recipient
        self.token_balance(&recipient).update(|balance| *balance += &amount);
    }

    // Storage getters
    #[view(getTokenBalance)]
    fn get_token_balance(&self, address: ManagedAddress) -> BigUint {
        self.token_balance(&address).get()
    }

    #[view(getTokenSupply)]
    fn get_token_supply(&self) -> BigUint {
        self.token_supply().get()
    }

    // Storage mappers
    #[storage_mapper("tokenSupply")]
    fn token_supply(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("tokenBalance")]
    fn token_balance(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("tokenName")]
    fn token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("tokenTicker")]
    fn token_ticker(&self) -> SingleValueMapper<ManagedBuffer>;
}