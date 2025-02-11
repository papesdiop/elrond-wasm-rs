use super::contract_impl::contract_implementation;
use crate::parse::parse_contract_trait;
use crate::preprocessing::trait_preprocessing;
use crate::validate::validate_contract;

pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let new_input = trait_preprocessing(input);
    let args_input = parse_macro_input!(args as syn::AttributeArgs);
    let proc_input = &parse_macro_input!(new_input as syn::ItemTrait);

    let contract = parse_contract_trait(args_input, proc_input);
    validate_contract(&contract);

    let contract_impl = contract_implementation(&contract, true);

    proc_macro::TokenStream::from(contract_impl)
}
