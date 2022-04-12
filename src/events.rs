elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait Events {
    #[event("bet_event")]
    fn bet_event(
        &self,
        #[indexed] id: usize,
        #[indexed] address: ManagedAddress<Self::Api>,
        #[indexed] amount: BigUint<Self::Api>,
        #[indexed] timestamp: u64,
        #[indexed] temperature: u64,
        #[indexed] humidity: u64,
        #[indexed] uv_level: u16,
    );
}