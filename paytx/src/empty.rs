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
        // Payment amount must be greater than 0
        require!(payment_amount > 0, "Payment amount must be greater than 0");

        self.payments().insert(payment_id, payment_amount);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn pay(&self, payment_id: u64) {
        let payment_amount = self.payments().get(&payment_id).unwrap_or_default();

        // Check if payment_amount is equal to the amount sent
        require!(payment_amount != 0, "Payment ID does not exist");
        
        // Check if payment_amount is equal to the amount sent
        require!(payment_amount == self.call_value().egld_value(), "Payment amount does not match");

        // Send from caller to payment account payment_amount EGLD
        self.send().direct_egld(&self.payment_account().get(), &self.call_value().egld_value());

        // Remove payment from payments map
        self.payments().remove(&payment_id);
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
