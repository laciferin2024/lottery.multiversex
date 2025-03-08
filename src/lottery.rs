#![no_std]

mod amm;
mod token;

use core::clone::Clone;
#[allow(unused_imports)]
use multiversx_sc::imports::*;
use crate::__wasm__endpoints__::current_game_id;

#[multiversx_sc::contract]
pub trait Lottery: token::LotteryToken + amm::LotteryAMM {
    #[init]
    fn init(&self, num_participants: usize, opt_token_id: OptionalValue<EgldOrEsdtTokenIdentifier>) -> () {
        // Default token information
        let token_name = ManagedBuffer::from("LotteryToken");
        let token_ticker = ManagedBuffer::from("LTRY");
        let initial_supply = BigUint::from(1_000_000u64);

        // Initialize token module
        self.init_token(initial_supply.clone(), token_name, token_ticker.clone()); //TODO: inbuilt token system and AMM


        let token_id = match opt_token_id {
            OptionalValue::Some(t) => t,
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };

        // Get token ID (using the ticker as an approximation since we're creating the token in-contract)
        // let token_id = TokenIdentifier::from_esdt_bytes("LTRY-94ac38");
        // let token_id = TokenIdentifier::from_esdt_bytes(self.token_ticker());

        // Default AMM settings
        let fee_percent = 30u64; // 0.3% fee

        // Initialize AMM module
        // self.init_amm(token_id, fee_percent); //FIXME: init amm

        // Default lottery settings
        // let num_participants = 1usize;
        let bet_amount = BigUint::from(10u64);

        // Initialize lottery
        self.init_lottery(token_id, num_participants, bet_amount);
    }

    fn init_lottery(
        &self,
        token_id: EgldOrEsdtTokenIdentifier,
        num_participants: usize,
        bet_amount: BigUint,
    ) {
        self.token_id().set(&token_id);
        self.num_participants().set(num_participants);
        self.bet_amount().set(&bet_amount);
        self.game_active().set(true);
        self.current_game_id().set(1);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint]
    fn place_bet(&self, chosen_number: u8) {
        // Check if the game is active
        require!(self.game_active().get(), "Game is not active");

        // Check if the bet number is valid (0-9)
        require!(chosen_number <= 9, "Number must be between 0 and 9");


        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();

        let token_id = self.token_id().get();
        let bet_amount = self.bet_amount().get();

        // Validate payment
        require!(
            payment_token == token_id,
            "Wrong token used for payment"
        );
        require!(payment_amount == bet_amount, "Wrong amount sent for bet");

        // Get caller
        let caller = self.blockchain().get_caller();
        let current_game_id = self.current_game_id().get();

        // Check if player already participated in this game
        require!(
            !self.has_placed_bet(&current_game_id, &caller).get(),
            "Already placed a bet in this game"
        );

        // Record the bet
        self.participants(&current_game_id).push(&caller);
        self.player_numbers(&current_game_id, &caller)
            .set(chosen_number);
        self.has_placed_bet(&current_game_id, &caller).set(true);

        self.bet_event(&caller, &current_game_id);

        let participants = self.participants(&current_game_id);

        // If all participants have joined, draw the winner
        if participants.len() == self.num_participants().get() {
            self.draw_winner(current_game_id);
        }
    }

    #[view(getGameStatus)]
    fn get_game_status(&self) -> MultiValue3<bool, usize, usize> {
        let game_active = self.game_active().get();
        let current_game_id = self.current_game_id().get();
        let num_participants = self.num_participants().get();
        let current_participants = self.participants(&current_game_id).len();

        (game_active, current_participants, num_participants).into()
    }

    fn draw_winner(&self, game_id: u32) {
        sc_print!("draw winner:{}", game_id);
        // Generate random number (0-9)
        let mut rand_source = RandomnessSource::new();

        let random_number = rand_source.next_u8_in_range(0, 10); //TODO: should be checked

        // Store the winning number
        self.winning_number(&game_id).set(random_number);

        // Find winners
        let participants = self.participants(&game_id);
        let mut winners = ManagedVec::new() as ManagedVec<ManagedAddress>;

        // for participant in participants.iter() {
        //     let player_number = self.player_numbers(&game_id, &participant).get();
        //
        //     if player_number == random_number {
        //         winners.push(participant);
        //     }
        // }

        for i in 0..participants.len() {
            // FIXME: u can't do this as participants.get is fetching from blockchain but this is being called
            let participant = participants.get(i);
            let player_number = self.player_numbers(&game_id, &participant).get();

            if player_number == random_number {
                winners.push(participant.clone());
                self.winner_event(&participant, &game_id)
            }
        }

        // Calculate rewards
        let total_pot = &self.bet_amount().get() * &BigUint::from(participants.len());

        if !winners.is_empty() {
            // Distribute winnings
            let winner_count = winners.len();
            let prize_per_winner = &total_pot / winner_count as u32;

            for winner in winners.iter() {
                // Send tokens to winner
                // self.send()
                //     .direct_esdt(&winner, &self.token_id().get(), 0, &prize_per_winner);
                self.send().direct(&winner, &self.token_id().get(), 0, &prize_per_winner);
            }
        } else {
            // No winners, return the tokens to players
            let consolation_share = total_pot / participants.len() as u32;

            for participant in participants.iter() {
                // self.send()
                //     .direct_esdt(&participant, &self.token_id().get(), 0, &half_bet_amount);

                self.send().direct(&participant, &self.token_id().get(), 0, &consolation_share);
            }
        }

        // Reset for next game
        self.current_game_id().set(game_id + 1);
    }

    // Storage mappers
    #[view]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view]
    #[storage_mapper("numParticipants")]
    fn num_participants(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("betAmount")]
    fn bet_amount(&self) -> SingleValueMapper<BigUint>;

    #[view]
    #[storage_mapper("gameActive")]
    fn game_active(&self) -> SingleValueMapper<bool>;

    #[view]
    #[storage_mapper("currentGameId")]
    fn current_game_id(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("participants")]
    fn participants(&self, game_id: &u32) -> VecMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("playerNumbers")]
    fn player_numbers(&self, game_id: &u32, player: &ManagedAddress) -> SingleValueMapper<u8>;

    #[view]
    #[storage_mapper("hasPlacedBet")]
    fn has_placed_bet(&self, game_id: &u32, player: &ManagedAddress) -> SingleValueMapper<bool>;

    #[storage_mapper("winningNumber")]
    fn winning_number(&self, game_id: &u32) -> SingleValueMapper<u8>;


    // events

    #[event("betEvent")]
    fn bet_event(&self, #[indexed] user: &ManagedAddress, #[indexed] game_id: &u32);


    #[event("winnerEvent")]
    fn winner_event(&self, #[indexed] user: &ManagedAddress, #[indexed] game_id: &u32);
}
