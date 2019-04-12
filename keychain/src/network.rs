use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Network {
  code: u32
}

impl Network {
  #[cfg(feature = "cardano")]
  pub const CARDANO : Network = Network { code: 0x80000717 };

  #[cfg(feature = "ethereum")]
  pub const ETHEREUM : Network = Network { code: 0x8000003c };

  pub fn all() -> Vec<Network> {
    let mut types: Vec<Network> = Vec::new();
    #[cfg(feature = "cardano")]
    {
      types.push(Network::CARDANO);
    }
    #[cfg(feature = "ethereum")]
    {
      types.push(Network::ETHEREUM);
    }
    types
  }

  pub fn code(&self) -> u32 {
    self.code
  }
}

impl fmt::Display for Network {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Network({})", self.code);
  }
}