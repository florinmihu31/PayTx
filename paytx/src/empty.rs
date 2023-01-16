#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait PayTx {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn register_payment(&self, payment_id: u32, payment_amount: BigUint) {
        // Payment ID must be greater than 0
        require!(payment_id > 0, "Payment ID must be greater than 0");
        
        // Payment amount must be greater than 0
        require!(payment_amount > 0, "Payment amount must be greater than 0");

        let payment_account_storage = self.payments(payment_id, payment_amount);

        // Check if the payment details already exist
        require!(payment_account_storage.is_empty(), "Payment details already exist");

        // Insert the payment account into the storage
        payment_account_storage.set(&self.blockchain().get_caller());
    }

    #[endpoint]
    #[payable("EGLD")]
    fn pay(&self, payment_id: u32) {
        // Payment ID must be greater than 0
        require!(payment_id > 0, "Payment ID must be greater than 0");

        let payment_account_storage = self.payments(payment_id, self.call_value().egld_value());

        // Check if the storage is not empty
        require! (!payment_account_storage.is_empty(), "No payment details found");

        // Send the payment to the merchant
        self.send().direct_egld(&payment_account_storage.get(), &self.call_value().egld_value());

        // Remove the payment from the storage
        payment_account_storage.clear();
    }

    // Payment account hash of the merchant
    #[storage_mapper("payments")]
    fn payments(&self, payment_id: u32, payment_amount: BigUint) -> SingleValueMapper<ManagedAddress>;
}
