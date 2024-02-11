use serde::{Deserialize, Serialize};
use starknet_crypto::FieldElement;
use std::collections::HashSet;
use utoipa::ToSchema;

/// Contains all data used in one round
#[derive(Debug, Clone)]
pub struct RoundTreeData {
    /// Which round
    pub round: u8,
    /// Cumulative amounts for each address in a Merkle tree
    pub tree: MerkleTree,
}

// Used for some intermediary calculations
pub struct RoundAmounts {
    pub round: u8,
    pub amounts: Vec<JSONAirdrop>,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Node,
    pub airdrops: Vec<CumulativeAirdrop>,
}

/// Calldata to be used for the associated Cairo contract
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CairoCalldata {
    /// The amount to claim
    pub amount: String,
    /// Merkle proof for the claim
    pub proof: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_addresses: HashSet<FieldElement>,
    pub value: FieldElement,
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
    pub cumulative_amount: u128,
}

#[derive(Debug, Clone)]
pub struct FileNameInfo {
    pub round: u8,
    pub full_path: String,
}
