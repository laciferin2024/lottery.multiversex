// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct LotteryProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for LotteryProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = LotteryProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        LotteryProxyMethods { wrapped_tx: tx }
    }
}

pub struct LotteryProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> LotteryProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<usize>,
    >(
        self,
        num_participants: Arg0,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&num_participants)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LotteryProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LotteryProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn place_bet<
        Arg0: ProxyArg<u8>,
    >(
        self,
        chosen_number: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("place_bet")
            .argument(&chosen_number)
            .original_result()
    }

    pub fn get_game_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue3<bool, usize, usize>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getGameStatus")
            .original_result()
    }

    pub fn token_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("token_id")
            .original_result()
    }

    pub fn num_participants(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("num_participants")
            .original_result()
    }

    pub fn bet_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("bet_amount")
            .original_result()
    }

    pub fn game_active(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("game_active")
            .original_result()
    }

    pub fn current_game_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("current_game_id")
            .original_result()
    }

    pub fn player_numbers<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        game_id: Arg0,
        player: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u8> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("player_numbers")
            .argument(&game_id)
            .argument(&player)
            .original_result()
    }

    pub fn has_placed_bet<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        game_id: Arg0,
        player: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("has_placed_bet")
            .argument(&game_id)
            .argument(&player)
            .original_result()
    }

    pub fn mint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        recipient: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("mint")
            .argument(&recipient)
            .argument(&amount)
            .original_result()
    }

    pub fn burn<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("burn")
            .argument(&amount)
            .original_result()
    }

    pub fn transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        recipient: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transfer")
            .argument(&recipient)
            .argument(&amount)
            .original_result()
    }

    pub fn get_token_balance<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTokenBalance")
            .argument(&address)
            .original_result()
    }

    pub fn get_token_supply(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTokenSupply")
            .original_result()
    }

    pub fn add_liquidity_egld<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        custom_token_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("add_liquidity_egld")
            .argument(&custom_token_amount)
            .original_result()
    }

    pub fn remove_liquidity<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        lp_token_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("remove_liquidity")
            .argument(&lp_token_amount)
            .original_result()
    }

    pub fn swap_egld_for_tokens(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("swap_egld_for_tokens")
            .original_result()
    }

    pub fn swap_tokens_for_egld(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("swap_tokens_for_egld")
            .original_result()
    }

    pub fn get_pool_info(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue4<TokenIdentifier<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPoolInfo")
            .original_result()
    }

    pub fn get_lp_balance<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLpBalance")
            .argument(&address)
            .original_result()
    }
}
