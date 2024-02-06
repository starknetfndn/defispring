use serde::Deserialize;
use starknet_crypto::{pedersen_hash, FieldElement};
use std::{collections::HashSet, fs::File, path::Path, str::FromStr, vec};

#[derive(Deserialize, Debug, Clone)]
pub struct Airdrop {
    pub address: String,
    amount: String,
}

pub struct MerkleTree {
    pub root: Node,
    airdrops: Vec<Airdrop>,
}

pub fn strip_leading_zeroes(hex: &str) -> String {
    if hex.len() <= 3 || &hex[..2] != "0x" {
        // len 3 is 0x0 -> do not remove this zero
        return hex.to_string();
    }
    let tmp: String = hex.to_lowercase().chars().skip(2).collect();
    let without_leading_zeroes = tmp.trim_start_matches('0');
    let res = format!("0x{}", without_leading_zeroes);
    match res.len() {
        // 0x0000 -> 0x -> return 0x0
        2 => "0x0".to_string(),
        _ => res,
    }
}

impl MerkleTree {
    pub fn new() -> Self {
        let airdrops = read_airdrop();
        let mut leaves: Vec<Node> = airdrops
            .clone()
            .into_iter()
            .map(|a| Node::new_leaf(a))
            .collect();

        // if odd length add a copy of last elem
        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let root = build_tree(leaves);

        MerkleTree { root, airdrops }
    }
    pub fn address_calldata(&self, address: &str) -> Result<Vec<String>, ()> {
        let felt_address = match FieldElement::from_str(address) {
            Ok(v) => v,
            _ => return Err(()),
        };
        if !&self.root.accessible_addresses.contains(&felt_address) {
            return Err(());
        }
        let mut hashes: Vec<FieldElement> = vec![];
        let mut current_node = &self.root;
        // if either child is_some, then both is_some
        loop {
            let left = current_node.left_child.as_ref().unwrap();
            let right = current_node.right_child.as_ref().unwrap();
            if left.accessible_addresses.contains(&felt_address) {
                hashes.push(right.value);
                current_node = left;
            } else {
                hashes.push(left.value);
                current_node = right;
            }
            if current_node.left_child.is_none() {
                break;
            }
        }
        // reverse to leaf first root last
        hashes = hashes.into_iter().rev().collect();

        let airdrop = self
            .airdrops
            .iter()
            .find(|a| &FieldElement::from_str(&a.address).unwrap() == &felt_address)
            .unwrap();

        let address = FieldElement::from_str(&airdrop.address).unwrap();
        let amount = FieldElement::from_str(&airdrop.amount).unwrap();

        let mut calldata = vec![address, amount];
        calldata.append(&mut hashes);

        // in order to be readable by FE needs to be base16 string
        Ok(calldata.iter().map(felt_to_b16).collect())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub accessible_addresses: HashSet<FieldElement>,
    pub value: FieldElement,
}

impl Node {
    fn new(a: Node, b: Node) -> Self {
        let (left_child, right_child) = match a.value.lt(&b.value) {
            true => (a, b),
            false => (b, a),
        };
        let value = hash(&left_child.value, &right_child.value);
        let mut accessible_addresses = HashSet::new();
        accessible_addresses.extend(left_child.accessible_addresses.clone());
        accessible_addresses.extend(right_child.accessible_addresses.clone());

        Node {
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
            accessible_addresses,
            value,
        }
    }
    fn new_leaf(airdrop: Airdrop) -> Self {
        let address = FieldElement::from_str(&strip_leading_zeroes(&airdrop.address)).unwrap();
        let amount = FieldElement::from_str(&airdrop.amount).unwrap();
        // keep order address, amount (cannot use fn hash)
        let value = pedersen_hash(&address, &amount);

        Node {
            left_child: None,
            right_child: None,
            accessible_addresses: vec![address].into_iter().collect(),
            value,
        }
    }
}

enum TreeBuilder {
    KeepGoing(Vec<Node>),
    Done(Node),
}

fn build_tree(leaves: Vec<Node>) -> Node {
    match build_tree_recursively(TreeBuilder::KeepGoing(leaves)) {
        TreeBuilder::Done(root) => return root,
        _ => unreachable!("Failed building the tree"),
    }
}

fn build_tree_recursively(tree_builder: TreeBuilder) -> TreeBuilder {
    let mut nodes = match tree_builder {
        TreeBuilder::KeepGoing(nodes) => nodes,
        _ => unreachable!("Failed building the tree"),
    };

    let mut next_nodes: Vec<Node> = vec![];

    while nodes.len() > 0 {
        let a = nodes.pop().unwrap();
        let b = nodes.pop().unwrap();
        next_nodes.push(Node::new(a, b));
    }

    if next_nodes.len() == 1 {
        // return root
        let root = next_nodes.pop().unwrap();
        return TreeBuilder::Done(root);
    }

    if next_nodes.len() % 2 == 1 {
        // if odd - pair last element with itself
        next_nodes.push(next_nodes.last().unwrap().clone());
    }

    build_tree_recursively(TreeBuilder::KeepGoing(next_nodes))
}

fn felt_to_b16(felt: &FieldElement) -> String {
    format!("{:#x}", felt)
}

pub fn hash(a: &FieldElement, b: &FieldElement) -> FieldElement {
    if a.lt(b) {
        return pedersen_hash(a, b);
    }
    pedersen_hash(b, a)
}

pub fn read_airdrop() -> Vec<Airdrop> {
    // path to "air-drop.json" relative to where the code was executed
    let possible_paths = vec![
        "carmine-api-airdrop/src/air-drop.json",
        "src/air-drop.json",
        "air-drop.json",
    ];
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