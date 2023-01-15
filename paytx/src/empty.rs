#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait PayTx {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn register_payment(&self, merchant_id: u32, payment_id: u32, payment_amount: BigUint) {
        // Merchant ID must be greater than 0
        require!(merchant_id > 0, "Merchant ID must be greater than 0");

        // Payment ID must be greater than 0
        require!(payment_id > 0, "Payment ID must be greater than 0");
        
        // Payment amount must be greater than 0
        require!(payment_amount > 0, "Payment amount must be greater than 0");

        // Check if the payment ID already exists
        require!(self.payments(merchant_id).get(&payment_id).is_none(), "Payment ID already exists");

        // Add the address corresponding to the merchant ID
        if self.payment_account(merchant_id).is_empty() {
            self.payment_account(merchant_id).set(&self.blockchain().get_caller());
        } else {
            // Check if the address corresponding to the merchant ID is the caller's address
            require!(self.payment_account(merchant_id).get() == self.blockchain().get_caller(), "Caller is not the merchant");
        }

        // Insert payment into the payments map corresponding to the merchant ID
        self.payments(merchant_id).insert(payment_id, payment_amount);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn pay(&self, merchant_id: u32, payment_id: u32) {
        // Merchant ID must be greater than 0
        require!(merchant_id > 0, "Merchant ID must be greater than 0");

        // Payment ID must be greater than 0
        require!(payment_id > 0, "Payment ID must be greater than 0");

        // Get payment amount from payments map
        let payment_amount = self.payments(merchant_id).get(&payment_id).unwrap_or_default();

        // Check if payment_amount is equal to the amount sent
        require!(payment_amount != 0, "Payment ID does not exist");
        
        // Check if payment_amount is equal to the amount sent
        require!(payment_amount == self.call_value().egld_value(), "Payment amount does not match");

        // Send from caller to payment account payment_amount EGLD
        self.send().direct_egld(&self.payment_account(merchant_id).get(), &self.call_value().egld_value());

        // Remove payment from payments map
        self.payments(merchant_id).remove(&payment_id);
    }

    // Map with payment ID as key and payment as value for each merchant
    #[storage_mapper("payments")]
    fn payments(&self, merchant_id: u32) -> MapMapper<u32, BigUint>;

    // Payment account hash of the merchant
    #[storage_mapper("payment_account")]
    fn payment_account(&self, merchant_id: u32) -> SingleValueMapper<ManagedAddress>;
}
