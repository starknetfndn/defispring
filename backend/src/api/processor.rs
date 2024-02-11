use regex::Regex;
use serde_json::from_slice;
use starknet_crypto::FieldElement;
use std::{collections::HashMap, fs::File, io::Read, path::Path};

use super::{
    data_storage::get_all_data,
    structs::{
        CumulativeAirdrop, FileNameInfo, JSONAirdrop, MerkleTree, RoundAmounts, RoundTreeData,
    },
};
use zip::ZipArchive;

pub fn get_raw_calldata(round: Option<u8>, address: &String) -> Result<Vec<String>, String> {
    let relevant_data = match get_round_data(round) {
        Ok(value) => value,
        Err(value) => {
            return Err(value);
        }
    };

    let calldata: Vec<String> = match relevant_data.tree.address_calldata(&address) {
        Ok(v) => v,
        Err(value) => {
            return Err(value);
        }
    };
    Ok(calldata)
}

pub fn get_raw_airdrop_amount(round: Option<u8>, address: &String) -> Result<u128, String> {
    let relevant_data = match get_round_data(round) {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    let drop = match relevant_data
        .tree
        .airdrops
        .iter()
        .find(|a| &a.address == address)
    {
        Some(v) => v,
        None => return Ok(0_128),
    };

    Ok(drop.cumulative_amount)
}

pub fn get_raw_root(round: Option<u8>) -> Result<FieldElement, String> {
    let relevant_data = match get_round_data(round) {
        Ok(value) => value,
        Err(_) => return Err("No data".to_string()), // TODO: check error message somehow?
    };
    Ok(relevant_data.tree.root.value)
}

// Gets data for a specific round
fn get_round_data(round: Option<u8>) -> Result<RoundTreeData, String> {
    let round_data = get_all_data();
    // Use round if it's provided. Otherwise use the latest round
    let use_round = match round {
        Some(v) => v,
        None => match round_data.iter().max_by_key(|&p| p.round) {
            None => return Err("No airdrop data found".to_string()),
            Some(p) => p.round,
        },
    };
    let relevant_data: Vec<RoundTreeData> = round_data
        .iter()
        .filter(|&p| p.round == use_round)
        .cloned()
        .collect();
    if relevant_data.len() != 1 {
        return Err("No airdrop data available".to_string());
    }
    Ok(relevant_data.get(0).unwrap().clone())
}

/// Temporary storage
struct RoundCumulativeMaps {
    round: u8,
    cumulative_amounts: HashMap<String, u128>,
}

/// Converts JSON airdrop data into cumulative tree+data per round
pub fn transform_airdrops_to_cumulative_rounds(
    mut airdrops: Vec<RoundAmounts>,
) -> Vec<RoundTreeData> {
    if airdrops.len() == 0 {
        return Vec::new();
    }
    airdrops.sort_by(|a, b| a.round.cmp(&b.round));

    let cumulative_amount_maps = map_cumulative_amounts(airdrops);

    let mut rounds: Vec<RoundTreeData> = Vec::new();
    for cum_map in cumulative_amount_maps.iter() {
        let mut curr_round_data: Vec<CumulativeAirdrop> = Vec::new();
        for key in cum_map.cumulative_amounts.keys() {
            let address_cumulative = CumulativeAirdrop {
                address: key.to_string(),
                cumulative_amount: cum_map.cumulative_amounts[key],
            };
            curr_round_data.push(address_cumulative);
        }

        if curr_round_data.len() > 0 {
            let tree = MerkleTree::new(curr_round_data);

            let round_drop = RoundTreeData {
                round: cum_map.round,
                tree: tree,
            };
            rounds.push(round_drop);
        }
    }

    rounds
}

/// Converts JSON airdrop data into cumulative map-per-round data
fn map_cumulative_amounts(airdrops: Vec<RoundAmounts>) -> Vec<RoundCumulativeMaps> {
    let mut all_rounds_cums: HashMap<String, u128> = HashMap::new();
    let mut round_maps: Vec<RoundCumulativeMaps> = Vec::new();

    for airdrop in airdrops.iter() {
        for data in airdrop.amounts.iter() {
            let amount = match data.amount.parse::<u128>() {
                Ok(value) => value,
                Err(_) => 0_u128, // TODO: what to do when data is invalid?
            };

            *all_rounds_cums
                .entry(data.address.clone())
                .or_insert_with(|| 0) += amount;
        }
        let map = RoundCumulativeMaps {
            round: airdrop.round,
            cumulative_amounts: all_rounds_cums.clone(),
        };

        round_maps.push(map);
    }

    round_maps
}

// Reads and accumulates all airdrop info for all rounds
pub fn read_airdrops() -> Vec<RoundTreeData> {
    let files = retrieve_valid_files();
    let mut round_amounts: Vec<RoundAmounts> = vec![];

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
            let airdrop: Vec<JSONAirdrop> =
                from_slice(&buffer).expect("Failed to deserialize airdrop");

            let round_amount = RoundAmounts {
                amounts: airdrop.clone(),
                round: file.round,
            };
            round_amounts.push(round_amount);
        }
    }
    transform_airdrops_to_cumulative_rounds(round_amounts)
}

/// Returns all files that have the correct filename syntax
fn retrieve_valid_files() -> Vec<FileNameInfo> {
    let mut valid_files: Vec<FileNameInfo> = vec![];
    let path = Path::new("src/raw_input");

    let template_pattern = r"^raw_(\d+)\.zip$";
    let regex = Regex::new(&template_pattern).expect("Invalid regex pattern");

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if let Some(captures) = regex.captures(entry.file_name().to_str().unwrap()) {
                // Collect valid file names
                if let Some(round) = captures.get(1) {
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

/// Retrieve allocated amount for an address in a specific round
impl RoundTreeData {
    pub fn address_amount(&self, address: &str) -> Result<u128, String> {
        let address_drop: Vec<CumulativeAirdrop> = self
            .tree
            .airdrops
            .iter()
            .filter(|a| &a.address == address)
            .cloned()
            .collect();

        if address_drop.len() == 0 {
            Ok(0_u128)
        } else {
            Ok(address_drop.get(0).unwrap().cumulative_amount)
        }
    }
}
