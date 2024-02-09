use std::collections::HashMap;

use defispring::api::{
    data::transform_airdrops_to_cumulative_rounds,
    structs::{JSONAirdrop, MerkleTree, RoundAmounts, RoundTreeData},
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

    let mut round_data: Vec<RoundAmounts> = Vec::new();
    round_data.push(RoundAmounts {
        round: 1u8,
        amounts: drop,
    });
    let res = transform_airdrops_to_cumulative_rounds(round_data);

    assert!(res[0].address_amount("0x1").unwrap() == 5_u128);
    assert!(res[0].address_amount("0x2").unwrap() == 6_u128);
    assert!(res[0].address_amount("0x3").unwrap() == 7_u128);
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

    let mut round_data: Vec<RoundAmounts> = Vec::new();
    round_data.push(RoundAmounts {
        round: 1u8,
        amounts: drop1,
    });
    round_data.push(RoundAmounts {
        round: 2u8,
        amounts: drop2,
    });
    let res = transform_airdrops_to_cumulative_rounds(round_data);

    assert!(res[0].address_amount("0x1").unwrap() == 5_u128);
    assert!(res[0].address_amount("0x2").unwrap() == 6_u128);
    assert!(res[0].address_amount("0x3").unwrap() == 7_u128);

    assert!(res[1].address_amount("0x1").unwrap() == 5_u128);
    assert!(res[1].address_amount("0x2").unwrap() == 6_u128);
    assert!(res[1].address_amount("0x3").unwrap() == 30_u128);
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

    let mut round_data: Vec<RoundAmounts> = Vec::new();
    round_data.push(RoundAmounts {
        round: 1u8,
        amounts: drop1,
    });
    round_data.push(RoundAmounts {
        round: 2u8,
        amounts: drop2,
    });
    round_data.push(RoundAmounts {
        round: 3u8,
        amounts: drop3,
    });
    let res = transform_airdrops_to_cumulative_rounds(round_data);

    assert!(res[0].address_amount("0x1").unwrap() == 5_u128);
    assert!(res[0].address_amount("0x2").unwrap() == 6_u128);
    assert!(res[0].address_amount("0x3").unwrap() == 7_u128);

    assert!(res[1].address_amount("0x1").unwrap() == 5_u128);
    assert!(res[1].address_amount("0x2").unwrap() == 6_u128);
    assert!(res[1].address_amount("0x3").unwrap() == 30_u128);

    assert!(res[2].address_amount("0x1").unwrap() == 5_u128);
    assert!(res[2].address_amount("0x2").unwrap() == 39_u128);
    assert!(res[2].address_amount("0x3").unwrap() == 30_u128);
}
