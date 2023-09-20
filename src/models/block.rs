use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block{
    pub index: u64,
    pub timestamp: u64,
    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
    pub log: String,
  
}
impl Block {

 // Create a new block. The hash will be calculated and set automatically.
pub fn new (
    index: u64,
    previous_hash: String,
   ) -> Self {
      // Current block to be created.      
      let mut block = Block {
         index: 0,
         timestamp: Utc::now().timestamp_millis() as u64,
         proof_of_work: u64::default(),
         previous_hash: String::default(),
         hash: String::default(),
         log: String::default(),
         
      };
      block
   }
// Mine block hash.
pub fn mine (&mut self, blockchain: Blockchain) {
    loop {
      if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
        self.proof_of_work += 1;
        self.hash = self.generate_block_hash();
        
      } else {
         break
      }
    }
  }
   // Calculate block hash.
   pub fn generate_block_hash(&self) -> String {
    let mut block_data = self.clone();
    block_data.hash = String::default();
    // Convert block to JSON format.
    let serialized_block_data = serde_json::to_string(&block_data).unwrap();
    // Calculate and return SHA-256 hash value.
    let mut hasher = Sha256::new();
    hasher.update(serialized_block_data);

    let result = hasher.finalize();
    
    format!("{:x}", result)
}
  //this function will pass a system command to a shell and append the output to the log varaiable
    pub fn get_audit_data(&self) -> String {
    let mut audit_data = self.clone();
    audit_data.log = String::default();
        // initializes the command method 
    let syscom = Command::new("ausearch")
     .arg("-st")
     .arg("recent")
     .arg("-F") 
     .arg("/home/sysadmin/project4/EVIDENCE")
     // this bit of code gets the standard output of the ausearch command
     .stdout(Stdio::piped())
     .output()
     .unwrap();
//converts bits from stdout to a string.
        let audit_data = String::from_utf8(syscom.stdout).unwrap();
audit_data
}

}
