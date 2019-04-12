
// External crates
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate cryptoxide;
extern crate rand;

// Internal modules
mod data;
mod key_factory;
mod key;
mod keychain;
mod manager;
mod key_path;
mod mnemonic;
mod network;
mod entropy;
mod error;


// Public Modules
pub mod networks;
pub mod bip39;
pub mod util;

//Exports
//pub use wallet::HDWallet;
pub use network::Network;
pub use manager::KeychainManager;
pub use key_path::GenericKeyPath;

#[cfg(feature = "custom-networks")]
pub use key_path::KeyPath;
pub use entropy::*;