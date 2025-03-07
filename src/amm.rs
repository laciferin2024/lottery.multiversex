#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait AutomatedMarketMaker {
    #[init]
    fn init(
        &self,
        custom_token_id: TokenIdentifier,
        fee_percent: u64
    ) {
        require!(fee_percent <= 1000, "Fee percent too high"); // Max 10%
        
        self.custom_token_id().set(&custom_token_id);
        self.fee_percent().set(fee_percent);
        self.lp_token_supply().set(&BigUint::zero());
        self.egld_reserve().set(&BigUint::zero());
        self.token_reserve().set(&BigUint::zero());
    }

    #[upgrade]
    fn upgrade(&self) {}

    // Add liquidity with EGLD and custom token
    #[payable("EGLD")]
    #[endpoint]
    fn add_liquidity_egld(&self, custom_token_amount: BigUint) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let egld_amount = self.call_value().egld_value();

        // Calculate liquidity tokens to mint
        let lp_tokens = if self.lp_token_supply().get() == BigUint::zero() {
            // First liquidity provision - use EGLD amount as initial LP token amount
            egld_amount.clone_value()
        } else {
            // Determine LP tokens based on proportion of liquidity provided
            let egld_reserve = self.egld_reserve().get();
            let token_reserve = self.token_reserve().get();

            let egld_proportion = &egld_amount.clone_value() * &self.lp_token_supply().get() / &egld_reserve;
            let token_proportion = &custom_token_amount * &self.lp_token_supply().get() / &token_reserve;

            core::cmp::min(egld_proportion, token_proportion)
        };

        // Transfer custom tokens from user to contract
        self.send().esdt_local_exec_accept_payment(
            &caller,
            &self.custom_token_id().get(),
            0,
            &custom_token_amount
        );

        // Update reserves
        self.egld_reserve().update(|reserve| *reserve += &egld_amount.clone_value());
        self.token_reserve().update(|reserve| *reserve += &custom_token_amount);

        // Mint LP tokens to user
        self.lp_token_balance(&caller).update(|balance| *balance += &lp_tokens);
        self.lp_token_supply().update(|supply| *supply += &lp_tokens);

        Ok(())
    }

    // Remove liquidity
    #[endpoint]
    fn remove_liquidity(&self, lp_token_amount: BigUint) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let caller_lp_balance = self.lp_token_balance(&caller).get();

        require!(caller_lp_balance >= lp_token_amount, "Insufficient LP tokens");

        // Calculate proportion of liquidity to return
        let total_lp_supply = self.lp_token_supply().get();
        let proportion = &lp_token_amount * &BigUint::from(10000u32) / &total_lp_supply;

        let egld_reserve = self.egld_reserve().get();
        let token_reserve = self.token_reserve().get();

        let egld_amount = &proportion * &egld_reserve / BigUint::from(10000u32);
        let token_amount = &proportion * &token_reserve / BigUint::from(10000u32);

        // Burn LP tokens
        self.lp_token_balance(&caller).update(|balance| *balance -= &lp_token_amount);
        self.lp_token_supply().update(|supply| *supply -= &lp_token_amount);

        // Update reserves
        self.egld_reserve().update(|reserve| *reserve -= &egld_amount);
        self.token_reserve().update(|reserve| *reserve -= &token_amount);

        // Transfer tokens back to user
        self.send().direct_egld(&caller, &egld_amount);
        self.send().direct_esdt(
            &caller,
            &self.custom_token_id().get(),
            0,
            &token_amount
        );

        Ok(())
    }

    // Swap EGLD for tokens
    #[payable("EGLD")]
    #[endpoint]
    fn swap_egld_for_tokens(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value();

        let egld_reserve = self.egld_reserve().get();
        let token_reserve = self.token_reserve().get();

        require!(egld_reserve > BigUint::zero() && token_reserve > BigUint::zero(), "Liquidity too low");

        // Calculate output amount using constant product formula with fee
        let fee_percent = self.fee_percent().get();
        let input_with_fee = &payment_amount.clone_value() * &BigUint::from(10000u32 - fee_percent);
        let numerator = &input_with_fee * &token_reserve;
        let denominator = &egld_reserve * &BigUint::from(10000u32) + &input_with_fee;
        let tokens_out = numerator / denominator;

        require!(tokens_out > BigUint::zero(), "Output amount too small");

        // Update reserves
        self.egld_reserve().update(|reserve| *reserve += &payment_amount.clone_value());
        self.token_reserve().update(|reserve| *reserve -= &tokens_out);

        // Send tokens to caller
        self.send().direct_esdt(
            &caller,
            &self.custom_token_id().get(),
            0,
            &tokens_out
        );

        Ok(())
    }

    // Swap tokens for EGLD
    #[endpoint]
    fn swap_tokens_for_egld(&self){
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.custom_token_id().get(),
            "Wrong token provided"
        );

        let egld_reserve = self.egld_reserve().get();
        let token_reserve = self.token_reserve().get();

        require!(egld_reserve > BigUint::zero() && token_reserve > BigUint::zero(), "Liquidity too low");

        // Calculate output amount using constant product formula with fee
        let fee_percent = self.fee_percent().get();
        let input_with_fee = &payment.amount * &BigUint::from(10000u32 - fee_percent);
        let numerator = &input_with_fee * &egld_reserve;
        let denominator = &token_reserve * &BigUint::from(10000u32) + &input_with_fee;
        let egld_out = numerator / denominator;

        require!(egld_out > BigUint::zero(), "Output amount too small");
        
        // Update reserves
        self.token_reserve().update(|reserve| *reserve += &payment.amount);
        self.egld_reserve().update(|reserve| *reserve -= &egld_out);
        
        // Send EGLD to caller
        self.send().direct_egld(&caller, &egld_out);
    }
    
    // Views
    #[view(getPoolInfo)]
    fn get_pool_info(&self) -> MultiValue4<TokenIdentifier, BigUint, BigUint, BigUint> {
        (
            self.custom_token_id().get(),
            self.egld_reserve().get(),
            self.token_reserve().get(),
            self.lp_token_supply().get()
        ).into()
    }
    
    #[view(getLpBalance)]
    fn get_lp_balance(&self, address: ManagedAddress) -> BigUint {
        self.lp_token_balance(&address).get()
    }
    
    // Storage mappers
    #[storage_mapper("customTokenId")]
    fn custom_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    
    #[storage_mapper("feePercent")]
    fn fee_percent(&self) -> SingleValueMapper<u64>;
    
    #[storage_mapper("egldReserve")]
    fn egld_reserve(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("tokenReserve")]
    fn token_reserve(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("lpTokenSupply")]
    fn lp_token_supply(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("lpTokenBalance")]
    fn lp_token_balance(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}