use crate::api::ErrorApi;
use crate::err_msg;
use crate::types::BoxedBytes;
use alloc::vec::Vec;

use super::Handle;

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
pub trait EndpointArgumentApi: ErrorApi {
    fn get_num_arguments(&self) -> i32;

    fn check_num_arguments(&self, expected: i32) {
        let nr_args = self.get_num_arguments();
        if nr_args != expected {
            self.signal_error(err_msg::ARG_WRONG_NUMBER);
        }
    }

    fn get_argument_len(&self, arg_index: i32) -> usize;

    fn copy_argument_to_slice(&self, arg_index: i32, slice: &mut [u8]);

    fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8>;

    fn get_argument_boxed_bytes(&self, arg_index: i32) -> BoxedBytes {
        self.get_argument_vec_u8(arg_index).into()
    }

    fn get_argument_big_int_raw(&self, arg_id: i32) -> Handle;

    fn get_argument_big_uint_raw(&self, arg_id: i32) -> Handle;

    fn get_argument_managed_buffer_raw(&self, arg_id: i32) -> Handle;

    fn get_argument_u64(&self, arg_id: i32) -> u64;

    fn get_argument_i64(&self, arg_id: i32) -> i64;
}
