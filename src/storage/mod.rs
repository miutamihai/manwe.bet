elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod bet;
pub use bet::Bet;

#[elrond_wasm::module]
pub trait Storage {
    #[view(getBets)]
    #[storage_mapper("bets")]
    fn bets(&self) -> VecMapper<Bet<Self::Api>>;
}