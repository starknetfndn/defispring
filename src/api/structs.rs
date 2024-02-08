use serde::Deserialize;
use starknet_crypto::FieldElement;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RoundTreeData {
    pub round: u8,
    pub protocol_id: u8,
    pub tree: MerkleTree,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Node,
    pub airdrops: Vec<Airdrop>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_addresses: HashSet<FieldElement>,
    pub value: FieldElement,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Airdrop {
    pub address: String,
    pub amount: String,
}
