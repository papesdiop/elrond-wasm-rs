elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedBufferFeatures {
    #[endpoint]
    fn mbuffer_overwrite(&self, mb1: ManagedBuffer, bytes: &[u8]) -> ManagedBuffer {
        let mut result = mb1;
        result.overwrite(bytes);
        result
    }

    #[endpoint]
    fn mbuffer_concat_1(&self, mb1: ManagedBuffer, mb2: ManagedBuffer) -> ManagedBuffer {
        let mut result = mb1;
        result.append(&mb2);
        result
    }

    #[endpoint]
    fn mbuffer_concat_2(&self, mb: ManagedBuffer, bytes: &[u8]) -> ManagedBuffer {
        let mut result = mb;
        result.append_bytes(bytes);
        result
    }

    #[endpoint]
    fn mbuffer_slice_1(
        &self,
        mb: ManagedBuffer,
        starting_position: usize,
        slice_len: usize,
    ) -> OptionalResult<BoxedBytes> {
        let mut result = BoxedBytes::zeros(slice_len);
        if mb
            .load_slice(starting_position, result.as_mut_slice())
            .is_ok()
        {
            OptionalResult::Some(result)
        } else {
            OptionalResult::None
        }
    }

    #[endpoint]
    fn mbuffer_slice_2(
        &self,
        mb: ManagedBuffer,
        starting_position: usize,
        slice_len: usize,
    ) -> OptionalResult<ManagedBuffer> {
        mb.copy_slice(starting_position, slice_len).into()
    }
}
