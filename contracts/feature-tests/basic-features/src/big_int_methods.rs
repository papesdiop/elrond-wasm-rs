elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait BigIntMethods {
    #[endpoint]
    fn sqrt_big_uint(&self, a: BigUint) -> BigUint {
        a.sqrt()
    }

    #[endpoint]
    fn sqrt_big_uint_ref(&self, a: &BigUint) -> BigUint {
        a.sqrt()
    }

    #[endpoint]
    fn log2_big_uint(&self, a: BigUint) -> u32 {
        a.log2()
    }

    #[endpoint]
    fn log2_big_uint_ref(&self, a: &BigUint) -> u32 {
        a.log2()
    }

    #[endpoint]
    fn pow_big_int(&self, a: BigInt, b: u32) -> BigInt {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_int_ref(&self, a: &BigInt, b: u32) -> BigInt {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_uint(&self, a: BigUint, b: u32) -> BigUint {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_uint_ref(&self, a: &BigUint, b: u32) -> BigUint {
        a.pow(b)
    }

    #[endpoint]
    fn big_uint_to_u64(&self, bu: &BigUint) -> OptionalResult<u64> {
        bu.to_u64().into()
    }

    #[endpoint]
    fn big_int_to_i64(&self, bi: &BigInt) -> OptionalResult<i64> {
        bi.to_i64().into()
    }
}
