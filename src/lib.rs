#![no_std]

elrond_wasm::imports!();

mod storage;

#[elrond_wasm::derive::contract]
pub trait BetContract: crate::storage::Storage {
    #[init]
    fn init(&self) {}
}
