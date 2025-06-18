use serde::{Serialize, Deserialize};
use std::hash::{DefaultHasher, Hasher, Hash};

pub mod components;
pub mod service;
pub mod plugin;
pub mod events;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_name: String,
    pub biography: String,
    pub identifier: String, // orange identity
    pub blocked_dids: Vec<String>,
    // Bitcoin Wallet Associated???
}

pub fn generate_name(input: &str) -> String {
    let (adjectives, nouns) = load_words();
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    let adj_index = ((hash % u16::MAX as u64) as usize) % adjectives.len();
    let noun_index = ((hash / u16::MAX as u64) as usize) % nouns.len();
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

fn load_words() -> (Vec<String>, Vec<String>) {
    let adjectives = include_str!("../assets/adjectives.txt")
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    let nouns = include_str!("../assets/animals.txt")
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    (adjectives, nouns)
}
