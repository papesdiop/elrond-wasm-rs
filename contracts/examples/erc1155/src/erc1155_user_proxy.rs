elrond_wasm::imports!();

#[elrond_wasm::proxy]
pub trait Erc1155UserProxy {
    #[endpoint(onERC1155Received)]
    fn on_erc1155_received(
        &self,
        operator: Address,
        from: Address,
        type_id: BigUint,
        value: BigUint,
        data: &[u8],
    ) -> SCResult<()>;

    #[endpoint(onERC1155BatchReceived)]
    fn on_erc1155_batch_received(
        &self,
        operator: Address,
        from: Address,
        type_ids: Vec<BigUint>,
        values: Vec<BigUint>,
        data: &[u8],
    ) -> SCResult<()>;
}
