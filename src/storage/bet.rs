elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, NestedEncode, NestedDecode, Debug)]
pub struct Bet<M: ManagedTypeApi> {
    pub(crate) address: ManagedAddress<M>,
    pub(crate) amount: BigUint<M>,
    pub(crate) timestamp: u64,
    pub(crate) temperature: f32,
    pub(crate) humidity: f32,
    pub(crate) uv_level: u16,
}