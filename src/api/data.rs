use lazy_static::lazy_static;
use regex::Regex;
use serde_json::from_slice;
use starknet_crypto::FieldElement;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    sync::RwLock,
    vec,
};

use super::structs::{Airdrop, MerkleTree, RoundTreeData};
use zip::ZipArchive;

// Use RwLock to allow for mutable access to the data
lazy_static! {
    static ref ROUND_DATA: RwLock<Vec<RoundTreeData>> = RwLock::new(Vec::new());
}

pub fn get_all_data() -> Vec<RoundTreeData> {
    ROUND_DATA
        .read()
        .expect("Failed to acquire read lock")
        .clone()
}

pub fn update_api_data() {
    let mut data = ROUND_DATA.write().expect("Failed to acquire write lock");

    let drops = read_airdrops();

    *data = drops;
}

pub fn get_raw_calldata(round: u8, address: &String) -> Vec<String> {
    let relevant_data = match get_round_data(round) {
        Ok(value) => value,
        Err(_) => return Vec::new(), // TODO: check error message somehow?
    };

    let calldata: Vec<String> = match relevant_data.tree.address_calldata(round, &address) {
        Ok(v) => v,
        Err(_) => vec![],
    };
    calldata
}

pub fn get_raw_airdrop_amount(round: u8, address: &String) -> u128 {
    let relevant_data = match get_round_data(round) {
        Ok(value) => value,
        Err(_) => return 0_u128, // TODO: check error message somehow?
    };

    let drop = match relevant_data
        .tree
        .airdrops
        .iter()
        .find(|a| &a.address == address)
    {
        Some(v) => v,
        None => return 0_u128,
    };

    drop.amount.parse::<u128>().unwrap()
}

pub fn get_raw_root(round: u8) -> Result<FieldElement, String> {
    let relevant_data = match get_round_data(round) {
        Ok(value) => value,
        Err(_) => return Err("No data".to_string()), // TODO: check error message somehow?
    };
    Ok(relevant_data.tree.root.value)
}

// Gets data for a specific round
fn get_round_data(round: u8) -> Result<RoundTreeData, String> {
    let round_data = get_all_data();
    let max_round = match round_data.iter().max_by_key(|&p| p.round) {
        None => return Err("No data".to_string()),
        Some(p) => p.round,
    };
    let mut use_round = round;
    if use_round == 0_u8 {
        use_round = max_round;
    }
    let relevant_data: Vec<RoundTreeData> = round_data
        .iter()
        .filter(|&p| p.round == use_round)
        .cloned()
        .collect();
    if relevant_data.len() != 1 {
        return Err("No data available".to_string());
    }
    Ok(relevant_data.get(0).unwrap().clone())
}

#[derive(Debug, Clone)]
pub struct FileNameInfo {
    round: u8,
    full_path: String,
}

// Reads all airdrop info for all rounds
pub fn read_airdrops() -> Vec<RoundTreeData> {
    let files = retrieve_valid_files();
    let mut results: Vec<RoundTreeData> = vec![];

    for file in files.iter() {
        let zipfile = File::open(file.clone().full_path).expect("Failed to open zip file");
        let mut archive: zip::ZipArchive<File> = ZipArchive::<File>::new(zipfile).unwrap();
        if archive.len() > 0 {
            // Only read the first file in the zip archive
            let mut archive_file = archive.by_index(0).unwrap();
            let mut buffer = Vec::new();
            archive_file
                .read_to_end(&mut buffer)
                .expect("problem reading zip");
            let airdrop: Vec<Airdrop> = from_slice(&buffer).expect("Failed to deserialize airdrop");

            let tree = MerkleTree::new(airdrop);
            let round_drop = RoundTreeData {
                round: file.round,
                tree: tree,
                cumulative_amounts: HashMap::new(),
            };
            results.push(round_drop);
        }
    }
    results
}

pub fn calculate_cumulative_amount(airdrop: &mut Vec<RoundTreeData>) {
    airdrop.sort_by(|a, b| a.round.cmp(&b.round));

    let mut all_rounds_cums: HashMap<String, u128> = HashMap::new();

    for data in airdrop.iter_mut() {
        for drop in data.tree.airdrops.iter_mut() {
            let amount = match drop.amount.parse::<u128>() {
                Ok(value) => value,
                Err(_) => 0_u128, // FIXME: what to do when data is invalid?
            };

            let current_amount = all_rounds_cums
                .entry(drop.address.clone())
                .or_insert(0_u128);

            *current_amount += amount;
        }
        // Take a snapshot of the current status and set or this round
        data.cumulative_amounts = all_rounds_cums.clone();
    }
    /*    for data in airdrop.iter() {
        for (address, cumulative_amount) in &data.cumulative_amounts {
            println!(
                "Round: {}, Address: {}, Cumulative Amount: {}",
                data.round, address, cumulative_amount
            );
        }
    } */
}

// Returns all files that have the correct filename syntax
fn retrieve_valid_files() -> Vec<FileNameInfo> {
    let mut valid_files: Vec<FileNameInfo> = vec![];
    let path = Path::new("src/raw_input");

    let template_pattern = r"^raw_(\d+)\.zip$";
    let regex = Regex::new(&template_pattern).expect("Invalid regex pattern");

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if let Some(captures) = regex.captures(entry.file_name().to_str().unwrap()) {
                if let Some(round) = captures.get(1) {
                    // TODO: what to do if filename is not correct?
                    let fileinfo = FileNameInfo {
                        full_path: entry.path().to_str().unwrap().to_string(),
                        round: round.as_str().parse::<u8>().unwrap(),
                    };
                    valid_files.push(fileinfo);
                }
            }
        }
    }
    valid_files
}
