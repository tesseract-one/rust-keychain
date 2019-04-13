use entropy::OsEntropyError;
use network::Network;
use data::{ Error as DataError };
use key::{ Error as KeyError };
use mnemonic::{ Error as MnemonicError };
use crypt::{ DecryptError as CryptError };
use std::error::{ Error as AnyError };
use std::fmt;

#[derive(Debug)]
pub enum Error {
  WrongPassword,
  NotEnoughData,
  CantCalculateSeedSize(usize, usize),
  InvalidSeedSize(usize),
  KeyDoesNotExist(Network),
  DataError(DataError),
  KeyError(Network, KeyError),
  EntropyGeneratorError(OsEntropyError),
  MnemonicError(MnemonicError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::WrongPassword => write!(f, "Wrong password"),
      &Error::NotEnoughData => write!(f, "Not enough data to load keychain"),
      &Error::CantCalculateSeedSize(min, max) => write!(f, "Can't calculate seed size for networks: min({}), max({})", min, max),
      &Error::InvalidSeedSize(size) => write!(f, "Invalid seed size {}", size),
      &Error::KeyDoesNotExist(nt) => write!(f, "Key for {} doesn't exist", nt),
      &Error::DataError(ref err) => write!(f, "Data parsing error {}", err),
      &Error::KeyError(ref nt, ref err) => write!(f, "Key error {} for network {}", err, nt),
      &Error::EntropyGeneratorError(ref err) => write!(f, "Entropy generator error {}", err),
      &Error::MnemonicError(ref err) => write!(f, "Mnemonic error {}", err)
    }
  }
}

impl AnyError for Error {}

impl From<OsEntropyError> for Error {
  fn from(err: OsEntropyError) -> Self {
    Error::EntropyGeneratorError(err)
  }
}

impl From<MnemonicError> for Error {
  fn from(err: MnemonicError) -> Self {
    Error::MnemonicError(err)
  }
}

impl From<DataError> for Error {
  fn from(err: DataError) -> Self {
    Error::DataError(err)
  }
}

impl From<CryptError> for Error {
  fn from(err: CryptError) -> Self {
    match err {
      CryptError::NotEnoughData => Error::NotEnoughData,
      CryptError::DecryptionFailed => Error::WrongPassword,
    }
  }
}

impl Error {
  pub fn from_key_error(net: &Network, err: KeyError) -> Self {
    Error::KeyError(net.clone(), err)
  }
}