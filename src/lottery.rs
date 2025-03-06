#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Lottery {
    #[init]
    fn init(
        &self, 
        token_id: TokenIdentifier, 
        num_participants: usize, 
        bet_amount: BigUint
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
        
        // Get payment info
        let payment = self.call_value().single_esdt();
        let token_id = self.token_id().get();
        let bet_amount = self.bet_amount().get();
        
        // Validate payment
        require!(payment.token_identifier == token_id, "Wrong token used for payment");
        require!(payment.amount == bet_amount, "Wrong amount sent for bet");
        
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
        self.player_numbers(&current_game_id, &caller).set(chosen_number);
        self.has_placed_bet(&current_game_id, &caller).set(true);
        
        // If all participants have joined, draw the winner
        if self.participants(&current_game_id).len() == self.num_participants().get() {
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
        // Generate random number (0-9)
        let random_bytes = self.crypto().random_bytes(1);
        let random_number = random_bytes[0] % 10;
        
        // Store the winning number
        self.winning_number(&game_id).set(random_number);
        
        // Find winners
        let participants = self.participants(&game_id);
        let mut winners = ManagedVec::new();
        
        for i in 0..participants.len() {
            let participant = participants.get(i);
            let player_number = self.player_numbers(&game_id, &participant).get();
            
            if player_number == random_number {
                winners.push(participant);
            }
        }
        
        // Calculate rewards
        let total_pot = &self.bet_amount().get() * &BigUint::from(participants.len());
        
        if !winners.is_empty() {
            // Distribute winnings
            let winner_count = winners.len();
            let prize_per_winner = &total_pot / winner_count;
            
            for winner in winners.iter() {
                // Send tokens to winner
                self.send().direct_esdt(
                    &winner,
                    &self.token_id().get(),
                    0,
                    &prize_per_winner
                );
            }
        } else {
            // No winners, return tokens to players
            for participant in participants.iter() {
                self.send().direct_esdt(
                    &participant,
                    &self.token_id().get(),
                    0,
                    &self.bet_amount().get()
                );
            }
        }
        
        // Reset for next game
        self.current_game_id().set(game_id + 1);
    }
    
    // Storage mappers
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    
    #[storage_mapper("numParticipants")]
    fn num_participants(&self) -> SingleValueMapper<usize>;
    
    #[storage_mapper("betAmount")]
    fn bet_amount(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("gameActive")]
    fn game_active(&self) -> SingleValueMapper<bool>;
    
    #[storage_mapper("currentGameId")]
    fn current_game_id(&self) -> SingleValueMapper<u32>;
    
    #[storage_mapper("participants")]
    fn participants(&self, game_id: &u32) -> VecMapper<ManagedAddress>;
    
    #[storage_mapper("playerNumbers")]
    fn player_numbers(&self, game_id: &u32, player: &ManagedAddress) -> SingleValueMapper<u8>;
    
    #[storage_mapper("hasPlacedBet")]
    fn has_placed_bet(&self, game_id: &u32, player: &ManagedAddress) -> SingleValueMapper<bool>;
    
    #[storage_mapper("winningNumber")]
    fn winning_number(&self, game_id: &u32) -> SingleValueMapper<u8>;
}