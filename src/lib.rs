#![no_std]

elrond_wasm::imports!();

mod bet;
mod events;
mod storage;
mod transactions;

#[elrond_wasm::derive::contract]
pub trait BetContract: storage::Storage + events::Events + transactions::Transactions {
    #[init]
    fn init(&self) {}
}
