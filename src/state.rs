use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Pool {
    pub total_balance: u64,
    pub contributions: HashMap<Pubkey, u64>,
    pub proposals: Vec<Proposal>,
    pub voting_active: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Proposal {
    pub recipient: Pubkey,
    pub votes: u64,
}
