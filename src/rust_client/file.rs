use solana_sdk::signer::keypair::Keypair;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

// pub fn read_keypair_file(path: PathBuf) -> Result<Keypair, Box<dyn std::error::Error>> {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let keypair_string = reader.lines().next()?;
//     let keypair = Keypair::from_str(&keypair_string)?;
//     Ok(keypair)
// }
