use lazy_static::lazy_static;
use regex::Regex;
use serde_json::from_slice;
use starknet_crypto::FieldElement;
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    sync::RwLock,
    vec,
};

use super::structs::{Airdrop, MerkleTree, Node};
use zip::ZipArchive;

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

pub fn update_api_data(round: u8) {
    let mut data = API_DATA.write().expect("Failed to acquire write lock");
    //data.root.value = FieldElement::from_str("14").unwrap();
    *data = MerkleTree::new(read_airdrop(round));
}

#[derive(Debug, Clone)]
struct FileNameInfo {
    full_path: String,
    file_name: String,
    protocol_id: u8,
}

pub fn read_airdrop(round: u8) -> Vec<Airdrop> {
    let files = extract_valid_files(round);

    // TODO: support for multiple files
    for file in files.iter() {
        let zipfile = File::open(file.clone().full_path).expect("Failed to open zip file");
        let mut archive: zip::ZipArchive<File> = ZipArchive::<File>::new(zipfile).unwrap();
        if archive.len() > 0 {
            // Only read the first file in the zip archive
            let mut file = archive.by_index(0).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("problem reading zip");
            let airdrop: Vec<Airdrop> = from_slice(&buffer).expect("Failed to deserialize airdrop");

            return airdrop;
        }
    }
    // TODO what to do if no data?
    let airdrop: Vec<Airdrop> = vec![];
    airdrop
}

fn extract_valid_files(round: u8) -> Vec<FileNameInfo> {
    let mut validFiles: Vec<FileNameInfo> = vec![];
    let path = Path::new("src/raw_input");

    let template_pattern = format!(r"^raw_{}_(\d+)\.zip$", round);
    let regex = Regex::new(&template_pattern).expect("Invalid regex pattern");

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            // println!("testing {:?}", entry.file_name().to_str().unwrap());
            if let Some(captures) = regex.captures(entry.file_name().to_str().unwrap()) {
                if let Some(protocol_id) = captures.get(1) {
                    // TODO: what to do if filename is not correct?
                    let fileinfo = FileNameInfo {
                        full_path: entry.path().to_str().unwrap().to_string(),
                        file_name: entry.file_name().to_str().unwrap().to_string(),
                        protocol_id: protocol_id.as_str().parse::<u8>().unwrap(),
                    };
                    //println!("VALID {:?} {:?}", entry.path(), protocol_id.as_str());
                    validFiles.push(fileinfo);
                }
            }
        }
    }
    validFiles
}
