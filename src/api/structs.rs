use serde::Deserialize;
use starknet_crypto::FieldElement;
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Debug, Clone)]
pub struct Airdrop {
    pub address: String,
    pub amount: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolAirdrop {
    pub airdrop: Vec<Airdrop>,
    pub protocol_id: u8,
    pub round: u8,
}

#[derive(Debug, Clone)]
pub struct RoundTreeData {
    pub round: u8,
    pub protocol_id: u8,
    pub tree: MerkleTree,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_addresses: HashSet<FieldElement>,
    pub value: FieldElement,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Node,
    pub airdrops: Vec<Airdrop>,
}
