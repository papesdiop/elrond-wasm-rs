use crate::api::ErrorApi;

use super::{BigIntApi, ManagedBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApi: BigIntApi + ManagedBufferApi + ErrorApi + Clone + 'static {
    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;
}
