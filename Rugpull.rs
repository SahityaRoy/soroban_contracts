extern crate soroban_sdk;
use soroban_sdk::prelude::*;

// Define the rug pull detection contract
#[contract]
pub fn RugPullDetection(
    #[state]
    total_balance: StorageValue<u64>,
    #[state]
    last_withdraw_time: StorageValue<u64>,
) {
    // Function to deposit funds
    #[constructor]
    fn deposit_funds(amount: u64) {
        let current_time = soroban_sdk::timestamp();
        if current_time - last_withdraw_time < 3600 {
            // Prevent deposits if a withdrawal occurred within the last hour
            panic!("Cannot deposit funds within one hour of withdrawal");
        }
        total_balance += amount;
    }

    // Function to withdraw funds
    fn withdraw_funds(amount: u64) {
        let current_time = soroban_sdk::timestamp();
        if current_time - last_withdraw_time < 3600 {
            // Prevent withdrawals if another withdrawal occurred within the last hour
            panic!("Cannot withdraw funds within one hour of another withdrawal");
        }
        if amount > total_balance / 2 {
            // If withdrawal amount is more than half of the total balance, consider it a potential rug pull
            panic!("Potential rug pull detected");
        }
        if amount > total_balance {
            panic!("Insufficient balance");
        }
        total_balance -= amount;
        last_withdraw_time = current_time;
    }
}
