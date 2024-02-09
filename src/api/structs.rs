use serde::Deserialize;
use starknet_crypto::FieldElement;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct RoundTreeData {
    pub round: u8,
    pub tree: MerkleTree,
    pub cumulative_amounts: HashMap<String, u128>,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Node,
    pub airdrops: Vec<CumulativeAirdrop>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_addresses: HashSet<FieldElement>,
    pub cumulated_amount: FieldElement,
}

// Data coming directly from raw JSONs
#[derive(Deserialize, Debug, Clone)]
pub struct JSONAirdrop {
    pub address: String,
    pub amount: String,
}

// Accumulated airdrop data
#[derive(Deserialize, Debug, Clone)]
pub struct CumulativeAirdrop {
    pub address: String,
    pub cumulative_amount: String,
}
