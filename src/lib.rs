use serde::{Serialize, Deserialize};

pub mod components;
pub mod service;
pub mod plugin;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_name: String,
    pub biography: String,
    pub identifier: String, // orange identity
    pub blocked_dids: Vec<String>,
    // Bitcoin Wallet Associated???
}

use sha2::{Digest, Sha256};
use std::fs;


pub fn generate_name(input: &str) -> String {
    let adjectives = load_words("assets/adjectives.txt");
    let nouns = load_words("assets/nouns.txt");
    let hash = Sha256::digest(input.as_bytes());
    let adj_index = (u16::from_be_bytes([hash[0], hash[1]]) as usize) % adjectives.len();
    let noun_index = (u16::from_be_bytes([hash[2], hash[3]]) as usize) % nouns.len();
    let adj = capitalize(&adjectives[adj_index]);
    let noun = capitalize(&nouns[noun_index]);
    format!("{}{}", adj, noun)
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn load_words(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}
