use serde::{Serialize, Deserialize};

pub mod components;
pub mod service;
pub mod plugin;
pub mod events;
pub mod pages;

use maverick_os::air::air;
pub use air::orange_name::OrangeName;

// #[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct Profile {
//     pub user_name: String,
//     pub biography: String,
//     pub identifier: String, // orange identity
//     pub blocked_dids: Vec<String>,
//     // Bitcoin Wallet Associated???
// }

