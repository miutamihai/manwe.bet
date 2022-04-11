elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bet::Bet;

#[elrond_wasm::module]
pub trait Events {
    #[event("bet_event")]
    fn bet_event(
        &self,
        #[indexed] bet: Bet<Self::Api> 
    );
}