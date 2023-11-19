// src/main.rs

use solana_program::pubkey::Pubkey;
use solana_lookup_tables::UserProfileLookup;

fn main() {
    // Initialize the lookup table
    let mut user_profile_lookup = UserProfileLookup::new();

    // Define user addresses and profile addresses
    let user1_address = Pubkey::new_unique();
    let user1_profile_address = Pubkey::new_unique();
    let user2_address = Pubkey::new_unique();
    let user2_profile_address = Pubkey::new_unique();

    // Populate the lookup table
    user_profile_lookup.add_user_profile(user1_address, user1_profile_address);
    user_profile_lookup.add_user_profile(user2_address, user2_profile_address);

    // Retrieve and print user profile addresses
    let current_user_address = user1_address;
    match user_profile_lookup.get_user_profile_address(&current_user_address) {
        Some(profile_address) => println!("User Profile Address: {:?}", profile_address),
        None => println!("User Profile Not Found"),
    }
}
