use serde::{Serialize, Deserialize};

pub mod components;
pub mod service;
pub mod plugin;
pub mod events;
pub mod pages;

use maverick_os::air::air;
pub use air::orange_name::OrangeName;
