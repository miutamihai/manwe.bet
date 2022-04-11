#![no_std]

elrond_wasm::imports!();

mod storage;

#[elrond_wasm::derive::contract]
pub trait BetContract: crate::storage::Storage {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, value: BigUint) {
        self.sum().set(value);
    }

    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
