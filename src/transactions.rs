elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bet::Bet;

#[elrond_wasm::module]
pub trait Transactions: crate::events::Events + crate::storage::Storage {
    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {}

    #[endpoint(placeBet)]
    #[payable("EGLD")]
    fn place_bet(
        &self,
        timestamp: u64,
        temperature: u64,
        humidity: u64,
        uv_level: u16,
        #[payment] payment: BigUint
    ) {
        let bet = Bet {
            timestamp: timestamp,
            temperature: temperature,
            humidity: humidity,
            uv_level: uv_level,
            address: self.blockchain().get_caller(),
            amount: payment.clone(),
        };

        self.bets().push(&bet);
        self.bet_event(self.bets().len(), bet.address, bet.amount, bet.timestamp, bet.temperature, bet.humidity, bet.uv_level);
    }

    #[only_owner]
    #[endpoint(winBet)]
    fn win_bet(&self, id: usize) {
        let bet = self.bets().get(id);

        self.bet_won(id);
        self.send().direct_egld(&bet.address, &bet.amount, &[]);
        self.bets().clear_entry(id);
    }

    #[only_owner]
    #[endpoint(cancelBet)]
    fn cancel_bet(&self, id: usize) {
        self.bet_cancelled(id);
        self.bets().clear_entry(id);
    }
}