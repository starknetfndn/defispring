use std::collections::HashMap;

use defispring::api::{
    data::calculate_cumulative_amount,
    structs::{JSONAirdrop, MerkleTree, RoundTreeData},
};

#[test]
fn test_cumulative_one_round() {
    let mut drop: Vec<JSONAirdrop> = vec![];
    drop.push(JSONAirdrop {
        address: "0x1".to_string(),
        amount: "5".to_string(),
    });
    drop.push(JSONAirdrop {
        address: "0x2".to_string(),
        amount: "6".to_string(),
    });
    drop.push(JSONAirdrop {
        address: "0x3".to_string(),
        amount: "7".to_string(),
    });

    let mut round_data: Vec<RoundTreeData> = Vec::new();
    round_data.push(RoundTreeData {
        round: 1u8,
        tree: MerkleTree::new(drop),
        cumulative_amounts: HashMap::new(),
    });
    calculate_cumulative_amount(&mut round_data);

    assert!(round_data[0].cumulative_amounts[&"0x1".to_string()] == 5_u128);
    assert!(round_data[0].cumulative_amounts[&"0x2".to_string()] == 6_u128);
    assert!(round_data[0].cumulative_amounts[&"0x3".to_string()] == 7_u128);
}

#[test]
fn test_cumulative_two_rounds() {
    let mut drop1: Vec<JSONAirdrop> = vec![];
    let mut drop2: Vec<JSONAirdrop> = vec![];
    drop1.push(JSONAirdrop {
        address: "0x1".to_string(),
        amount: "5".to_string(),
    });
    drop1.push(JSONAirdrop {
        address: "0x2".to_string(),
        amount: "6".to_string(),
    });
    drop1.push(JSONAirdrop {
        address: "0x3".to_string(),
        amount: "7".to_string(),
    });
    drop2.push(JSONAirdrop {
        address: "0x3".to_string(),
        amount: "23".to_string(),
    });

    let mut round_data: Vec<RoundTreeData> = Vec::new();
    round_data.push(RoundTreeData {
        round: 1u8,
        tree: MerkleTree::new(drop1),
        cumulative_amounts: HashMap::new(),
    });
    round_data.push(RoundTreeData {
        round: 2u8,
        tree: MerkleTree::new(drop2),
        cumulative_amounts: HashMap::new(),
    });
    calculate_cumulative_amount(&mut round_data);

    assert!(round_data[0].cumulative_amounts[&"0x1".to_string()] == 5_u128);
    assert!(round_data[0].cumulative_amounts[&"0x2".to_string()] == 6_u128);
    assert!(round_data[0].cumulative_amounts[&"0x3".to_string()] == 7_u128);

    assert!(round_data[1].cumulative_amounts[&"0x1".to_string()] == 5_u128);
    assert!(round_data[1].cumulative_amounts[&"0x2".to_string()] == 6_u128);
    assert!(round_data[1].cumulative_amounts[&"0x3".to_string()] == 30_u128);
}

#[test]
fn test_cumulative_three_rounds() {
    let mut drop1: Vec<JSONAirdrop> = vec![];
    let mut drop2: Vec<JSONAirdrop> = vec![];
    let mut drop3: Vec<JSONAirdrop> = vec![];
    drop1.push(JSONAirdrop {
        address: "0x1".to_string(),
        amount: "5".to_string(),
    });
    drop1.push(JSONAirdrop {
        address: "0x2".to_string(),
        amount: "6".to_string(),
    });
    drop1.push(JSONAirdrop {
        address: "0x3".to_string(),
        amount: "7".to_string(),
    });
    drop2.push(JSONAirdrop {
        address: "0x3".to_string(),
        amount: "23".to_string(),
    });
    drop3.push(JSONAirdrop {
        address: "0x2".to_string(),
        amount: "33".to_string(),
    });

    let mut round_data: Vec<RoundTreeData> = Vec::new();
    round_data.push(RoundTreeData {
        round: 1u8,
        tree: MerkleTree::new(drop1),
        cumulative_amounts: HashMap::new(),
    });
    round_data.push(RoundTreeData {
        round: 2u8,
        tree: MerkleTree::new(drop2),
        cumulative_amounts: HashMap::new(),
    });
    round_data.push(RoundTreeData {
        round: 3u8,
        tree: MerkleTree::new(drop3),
        cumulative_amounts: HashMap::new(),
    });
    calculate_cumulative_amount(&mut round_data);

    assert!(round_data[0].cumulative_amounts[&"0x1".to_string()] == 5_u128);
    assert!(round_data[0].cumulative_amounts[&"0x2".to_string()] == 6_u128);
    assert!(round_data[0].cumulative_amounts[&"0x3".to_string()] == 7_u128);

    assert!(round_data[1].cumulative_amounts[&"0x1".to_string()] == 5_u128);
    assert!(round_data[1].cumulative_amounts[&"0x2".to_string()] == 6_u128);
    assert!(round_data[1].cumulative_amounts[&"0x3".to_string()] == 30_u128);

    assert!(round_data[2].cumulative_amounts[&"0x1".to_string()] == 5_u128);
    assert!(round_data[2].cumulative_amounts[&"0x2".to_string()] == 39_u128);
    assert!(round_data[2].cumulative_amounts[&"0x3".to_string()] == 30_u128);
}
