use serde::{Serialize, Deserialize};

mod components;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_name: String,
    pub biography: String,
    pub identifier: String, // orange identity
    pub blocked_dids: Vec<String>,
    // Bitcoin Wallet Associated???
}

pub mod prelude {
    pub use crate::Profile;
    pub use crate::components::*;
}

