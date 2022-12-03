#![no_std]

elrond_wasm::imports!();

// use uuid::Uuid;

#[elrond_wasm::contract]
pub trait PayTx {
    #[init]
    fn init(&self) {
        // Set the payment address to the caller's address
        self.payment_account().set(&self.blockchain().get_caller());
    }

    #[endpoint]
    fn register_payment(&self, payment_id: u64, payment_amount: BigUint) {
        self.payments().insert(payment_id, payment_amount);
    }

    #[endpoint]
    fn get_value(&self, payment_id: u64) -> BigUint {
        self.payments().get(&payment_id).unwrap_or_default()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn pay(&self, payment_id: u64) -> SCResult<ManagedAddress> {
        let payment_amount = self.payments().get(&payment_id).unwrap_or_default();

        // Send from caller to payment account payment_amount EGLD
        self.send().direct_egld(&self.payment_account().get(), &self.call_value().egld_value());
        Ok(self.payment_account().get())
    }

    // Map with uuid as key and payment as value
    #[view(getPayments)]
    #[storage_mapper("payments")]
    fn payments(&self) -> MapMapper<u64, BigUint>;

    // Payment account hash
    #[view(getPaymentAccount)]
    #[storage_mapper("payment_account")]
    fn payment_account(&self) -> SingleValueMapper<ManagedAddress>;
}
