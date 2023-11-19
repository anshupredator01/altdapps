// src/lookup_table.rs

use solana_program::pubkey::Pubkey;
use std::collections::HashMap;

pub struct UserProfileLookup {
    pub user_address_to_profile: HashMap<Pubkey, Pubkey>,
}

impl UserProfileLookup {
    pub fn new() -> Self {
        Self {
            user_address_to_profile: HashMap::new(),
        }
    }

    pub fn add_user_profile(&mut self, user_address: Pubkey, profile_address: Pubkey) {
        self.user_address_to_profile.insert(user_address, profile_address);
    }

    pub fn get_user_profile_address(&self, user_address: &Pubkey) -> Option<&Pubkey> {
        self.user_address_to_profile.get(user_address)
    }
}
