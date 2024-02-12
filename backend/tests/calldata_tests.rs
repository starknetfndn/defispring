use std::collections::HashMap;

use defispring::api::{
    processor::{get_raw_calldata, transform_allocations_to_cumulative_rounds},
    structs::{JSONAllocation, MerkleTree, RoundAmounts, RoundTreeData},
};

#[test]
fn test_empty_data() {
    let mut drop: Vec<JSONAllocation> = vec![];
    drop.push(JSONAllocation {
        address: "0x1".to_string(),
        amount: "5".to_string(),
    });
    drop.push(JSONAllocation {
        address: "0x2".to_string(),
        amount: "6".to_string(),
    });

    let mut round_data: Vec<RoundAmounts> = Vec::new();
    round_data.push(RoundAmounts {
        round: 1u8,
        amounts: drop,
    });

    let res = get_raw_calldata(Some(1u8), &"0x1".to_string());
}
#[ignore]
#[test]
fn test_odd_data() {
    let mut drop: Vec<JSONAllocation> = vec![];
    drop.push(JSONAllocation {
        address: "".to_string(),
        amount: "0".to_string(),
    });
    drop.push(JSONAllocation {
        address: "0x1".to_string(),
        amount: "0".to_string(),
    });

    let mut round_data: Vec<RoundAmounts> = Vec::new();
    round_data.push(RoundAmounts {
        round: 1u8,
        amounts: drop,
    });
    let res = transform_allocations_to_cumulative_rounds(round_data);

    assert!(res[0].address_amount("0x1").unwrap() == 0_u128);
    assert!(res[0].address_amount("0x2").unwrap() == 0_u128);
}
