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

use super::structs::{Airdrop, MerkleTree, Node, ProtocolAirdrop, RoundTreeData};
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
    let mut all_data: Vec<RoundTreeData> = Vec::new();
    // round -> protocol_id -> MerkleTree
    let mut hashes: HashMap<u8, HashMap<u8, MerkleTree>> = HashMap::new();
    for drop in drops.iter() {
        let tree = MerkleTree::new(drop.airdrop.clone());
        let data = RoundTreeData {
            round: drop.round,
            protocol_id: drop.protocol_id,
            tree: tree,
        };
        all_data.push(data);
    }

    *data = all_data;
}

#[derive(Debug, Clone)]
pub struct FileNameInfo {
    full_path: String,
    file_name: String,
    protocol_id: u8,
    round: u8,
}

// Reads all airdrop info for the given round
pub fn read_airdrops() -> Vec<ProtocolAirdrop> {
    let files = extract_valid_files();
    let mut results: Vec<ProtocolAirdrop> = vec![];

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

            let protocol_drop = ProtocolAirdrop {
                airdrop: airdrop,
                protocol_id: file.protocol_id,
                round: file.round,
            };
            results.push(protocol_drop);
        }
    }
    results
}

// Returns all files that have the correct filename
fn extract_valid_files() -> Vec<FileNameInfo> {
    let mut validFiles: Vec<FileNameInfo> = vec![];
    let path = Path::new("src/raw_input");

    let template_pattern = r"^raw_(\d+)_(\d+)\.zip$";
    let regex = Regex::new(&template_pattern).expect("Invalid regex pattern");

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            // println!("testing {:?}", entry.file_name().to_str().unwrap());
            if let Some(captures) = regex.captures(entry.file_name().to_str().unwrap()) {
                if let Some(round) = captures.get(1) {
                    if let Some(protocol_id) = captures.get(2) {
                        // TODO: what to do if filename is not correct?
                        let fileinfo = FileNameInfo {
                            full_path: entry.path().to_str().unwrap().to_string(),
                            file_name: entry.file_name().to_str().unwrap().to_string(),
                            protocol_id: protocol_id.as_str().parse::<u8>().unwrap(),
                            round: round.as_str().parse::<u8>().unwrap(),
                        };
                        validFiles.push(fileinfo);
                    }
                }
            }
        }
    }
    validFiles
}
