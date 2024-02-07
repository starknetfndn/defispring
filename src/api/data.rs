use lazy_static::lazy_static;
use starknet_crypto::FieldElement;
use std::{collections::HashSet, fs::File, path::Path, str::FromStr, sync::RwLock, vec};

use super::structs::{Airdrop, MerkleTree, Node};

//use crate::api::merkle_tree::MerkleTree;
//use super::structs::{Airdrop, MerkleTree, Node};

// Use RwLock to allow for mutable access to the data
lazy_static! {
    static ref API_DATA: RwLock<MerkleTree> = RwLock::new(MerkleTree {
        airdrops: vec![],
        root: Node {
            left_child: None,
            right_child: None,
            accessible_addresses: HashSet::new(),
            value: FieldElement::from_str("0").unwrap()
        }
    });
}

pub fn get_api_data() -> MerkleTree {
    API_DATA
        .read()
        .expect("Failed to acquire read lock")
        .clone()
}

pub fn update_api_data() {
    let mut data = API_DATA.write().expect("Failed to acquire write lock");
    //data.root.value = FieldElement::from_str("14").unwrap();
    *data = MerkleTree::new(read_airdrop());
}

pub fn read_airdrop() -> Vec<Airdrop> {
    // path to "air-drop.json" relative to where the code was executed
    let possible_paths = vec!["src/air-drop.json"];
    let right_path_res = possible_paths.iter().find(|path| Path::new(path).exists());
    let right_path = match right_path_res {
        Some(v) => Path::new(v),
        None => panic!("Incorect airdrop file path"),
    };
    let file = File::open(right_path).expect("Failed to read file");
    let reader = std::io::BufReader::new(file);
    let airdrop: Vec<Airdrop> = serde_json::from_reader(reader).expect("Failed to parse airdrop");
    airdrop
}
