elrond_wasm::imports!();

use super::storage;

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[elrond_wasm::module]
pub trait ForwarderEsdtModule: storage::ForwarderStorageModule {
    #[view(getFungibleEsdtBalance)]
    fn get_fungible_esdt_balance(&self, token_identifier: &TokenIdentifier) -> BigUint {
        self.blockchain()
            .get_esdt_balance(&self.blockchain().get_sc_address(), token_identifier, 0)
    }

    #[endpoint]
    fn send_esdt(
        &self,
        to: &Address,
        token_id: TokenIdentifier,
        amount: &BigUint,
        #[var_args] opt_data: OptionalArg<BoxedBytes>,
    ) {
        let data = match &opt_data {
            OptionalArg::Some(data) => data.as_slice(),
            OptionalArg::None => &[],
        };
        self.send().direct(to, &token_id, 0, amount, data);
    }

    #[payable("*")]
    #[endpoint]
    fn send_esdt_with_fees(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        to: Address,
        percentage_fees: BigUint,
    ) {
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL.into();
        let amount_to_send = payment - fees;

        self.send().direct(&to, &token_id, 0, &amount_to_send, &[]);
    }

    #[endpoint]
    fn send_esdt_twice(
        &self,
        to: &Address,
        token_id: TokenIdentifier,
        amount_first_time: &BigUint,
        amount_second_time: &BigUint,
        #[var_args] opt_data: OptionalArg<BoxedBytes>,
    ) {
        let data = match &opt_data {
            OptionalArg::Some(data) => data.as_slice(),
            OptionalArg::None => &[],
        };
        self.send()
            .direct(to, &token_id, 0, amount_first_time, data);
        self.send()
            .direct(to, &token_id, 0, amount_second_time, data);
    }

    #[endpoint]
    fn send_esdt_direct_multi_transfer(
        &self,
        to: Address,
        #[var_args] token_payments: VarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = Vec::new();

        for multi_arg in token_payments.into_vec() {
            let (token_name, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment {
                token_name,
                token_nonce,
                amount,
                token_type: EsdtTokenType::Invalid, // not used
            };

            all_token_payments.push(payment);
        }

        let _ = self.send().direct_multi_esdt_transfer_execute(
            &to,
            &all_token_payments,
            self.blockchain().get_gas_left(),
            &[],
            &ArgBuffer::new(),
        );
    }

    #[payable("EGLD")]
    #[endpoint]
    fn issue_fungible_token(
        &self,
        #[payment] issue_cost: BigUint,
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: BigUint,
    ) -> AsyncCall<Self::SendApi> {
        let caller = self.blockchain().get_caller();

        ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
    }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &Address,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment] returned_tokens: BigUint,
        #[call_result] result: AsyncCallResult<()>,
    ) {
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            AsyncCallResult::Ok(()) => {
                self.last_issued_token().set(&token_identifier);
                self.last_error_message().clear();
            },
            AsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
    }

    #[endpoint]
    fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().esdt_local_mint(&token_identifier, 0, &amount);
    }

    #[endpoint]
    fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().esdt_local_burn(&token_identifier, 0, &amount);
    }
}
